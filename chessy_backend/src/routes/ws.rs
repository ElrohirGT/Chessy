use std::{fmt::Display, sync::Arc};

use actix::{fut, prelude::*};
use actix_broker::BrokerIssue;
use actix_web::{
    http::header::ContentType, web, Error, HttpRequest, HttpResponse, Responder, ResponseError,
};
use actix_web_actors::ws;
use chess_engine::BoardMovement;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    websocket::{
        ChessServer, CreateGame, GameMessage, JoinGame, JoinedGameResponses, LeaveGame,
        SendMovement,
    },
    AppState,
};

#[derive(Debug, thiserror::Error)]
pub enum WSConnectionError {
    #[error("Couldn't aquire the lock for users, it may be poisoned!")]
    InternalLockError,
    #[error("The user with the given id `{0}` doesn't exists!")]
    UserDoesntExists(Uuid),
    #[error("Internal error with websocket!")]
    WebError(Error),
}

impl ResponseError for WSConnectionError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            WSConnectionError::InternalLockError => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
            WSConnectionError::UserDoesntExists(_) => actix_http::StatusCode::BAD_REQUEST,
            WSConnectionError::WebError(_) => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_http::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }
}

pub async fn ws_endpoint(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<impl Responder, WSConnectionError> {
    let user_id = path.into_inner();
    log::debug!("User id `{}` wants to connect to websocket", user_id);

    let users = data
        .lock()
        .map_err(|_| WSConnectionError::InternalLockError)?;
    let username = users
        .get(&user_id)
        .ok_or_else(|| WSConnectionError::UserDoesntExists(user_id))?;

    let session = WsChatSession::new(user_id, username.clone());
    ws::start(session, &req, stream).map_err(WSConnectionError::WebError)
}

pub struct WsChatSession {
    id: Uuid,
    game_id: Option<Uuid>,
    username: Arc<str>,
}

impl WsChatSession {
    pub fn join_game(&mut self, ids: ClientAndGameId, ctx: &mut ws::WebsocketContext<Self>) {
        let ClientAndGameId { game_id, client_id } = ids;

        let client = ctx.address().recipient();
        let join_msg = JoinGame {
            game_id,
            client_id,
            client,
            name: self.username.clone(),
        };

        // Leave the current game.
        self.leave_game(ctx);

        ChessServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(move |result, actor, _| {
                if let Ok(JoinedGameResponses::JoinedGame) = result {
                    actor.id = client_id;
                    actor.game_id = Some(game_id);
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn create_game(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let client = ctx.address().recipient();
        let client_id = self.id;
        let msg = CreateGame {
            client_id,
            client,
            name: self.username.clone(),
        };

        // Leave the current game if there is one
        self.leave_game(ctx);

        ChessServer::from_registry()
            .send(msg)
            .into_actor(self)
            .then(move |result, actor, _| {
                if let Ok(game_id) = result {
                    actor.id = client_id.clone();
                    actor.game_id = Some(game_id);
                }

                fut::ready(())
            })
            .wait(ctx)
    }

    /// Tries to leave a game if we're connected to one.
    pub fn leave_game(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let client_id = self.id;
        match self.game_id {
            Some(game_id) => {
                let msg = LeaveGame { game_id, client_id };
                self.issue_system_sync(msg, ctx);
            }
            None => {}
        }
    }

    fn make_movement(
        &self,
        movement: BoardMovement,
        ctx: &mut ws::WebsocketContext<WsChatSession>,
    ) {
        let client_id = self.id;
        match self.game_id {
            Some(game_id) => {
                let msg = SendMovement {
                    game_id,
                    client_id,
                    movement,
                };

                self.issue_system_sync(msg, ctx);
            }
            None => ctx.text(WsSessionErrors::GameConnectionNeeded.to_string()),
        }
    }

    fn new(user_id: Uuid, username: Arc<str>) -> Self {
        Self {
            id: user_id,
            game_id: None,
            username,
        }
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!(
            "WsChatSession closed for {}({}) in room {:?}",
            self.username.clone(),
            self.id,
            self.game_id
        );
    }
}

impl Handler<GameMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: GameMessage, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&msg)
            .expect("The specified game message couldn't be converted to json.");
        ctx.text(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();

                match serde_json::from_str(msg) {
                    Ok(result) => match result {
                        WsSessionMessage::CreateGame => self.create_game(ctx),
                        WsSessionMessage::JoinGame(ids) => self.join_game(ids, ctx),
                        WsSessionMessage::LeaveGame => self.leave_game(ctx),
                        WsSessionMessage::Movement(movement) => self.make_movement(movement, ctx),
                    },
                    Err(_) => {
                        ctx.text(WsSessionErrors::InvalidJSONRequest.to_string());
                    }
                }
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientAndGameId {
    game_id: Uuid,
    client_id: Uuid,
}

/// Represents a message sent by the client to the server.
#[derive(Debug, Deserialize, Serialize)]
enum WsSessionMessage {
    CreateGame,
    JoinGame(ClientAndGameId),
    LeaveGame,
    Movement(BoardMovement),
}

#[derive(Debug, thiserror::Error)]
enum WsSessionErrors {
    #[error("ERROR: Invalid JSON format.")]
    InvalidJSONRequest,
    #[error("ERROR: Needs to be connected to a game before.")]
    GameConnectionNeeded,
}
