use std::sync::Arc;

use actix::{fut, prelude::*};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
use chess_engine::BoardMovement;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::websocket::{
    ChessServer, CreateGame, GameMessage, JoinGame, JoinedGameResponses, LeaveGame, SendMovement,
};

#[derive(Default)]
pub struct WsChatSession {
    id: Uuid,
    game_id: Uuid,
    username: Option<Arc<str>>,
}

impl WsChatSession {
    pub fn join_game(&mut self, ids: ClientAndGameId, ctx: &mut ws::WebsocketContext<Self>) {
        let ClientAndGameId { game_id, client_id } = ids;
        let leave_msg = LeaveGame { game_id, client_id };
        self.issue_system_sync(leave_msg, ctx);

        let client = ctx.address().recipient();
        let join_msg = JoinGame {
            game_id,
            client_id,
            client,
        };

        ChessServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(move |result, actor, _| {
                if let Ok(JoinedGameResponses::JoinedGame) = result {
                    actor.id = client_id;
                    actor.game_id = game_id;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn create_game(&mut self, client_id: Uuid, ctx: &mut ws::WebsocketContext<Self>) {
        let client = ctx.address().recipient();
        let msg = CreateGame { client_id, client };
        ChessServer::from_registry()
            .send(msg)
            .into_actor(self)
            .then(move |result, actor, _| {
                if let Ok(game_id) = result {
                    actor.id = client_id.clone();
                    actor.game_id = game_id;
                }

                fut::ready(())
            })
            .wait(ctx)
    }

    pub fn leave_game(&mut self, ids: ClientAndGameId, ctx: &mut ws::WebsocketContext<Self>) {
        let ClientAndGameId { game_id, client_id } = ids;
        let msg = LeaveGame { game_id, client_id };

        self.issue_system_sync(msg, ctx);
    }

    fn make_movement(
        &self,
        ids: ClientAndGameId,
        movement: BoardMovement,
        ctx: &mut ws::WebsocketContext<WsChatSession>,
    ) {
        let ClientAndGameId { game_id, client_id } = ids;
        let msg = SendMovement {
            game_id,
            client_id,
            movement,
        };

        self.issue_system_sync(msg, ctx);
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!(
            "WsChatSession closed for {}({}) in room {}",
            self.username.clone().unwrap_or_else(|| "anon".into()),
            self.id,
            self.game_id
        );
    }
}

impl Handler<GameMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: GameMessage, ctx: &mut Self::Context) {
        log::debug!("Received a GameMessage:\n{:?}", msg);
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

                let result: WsSessionMessage =
                    serde_json::from_str(msg).expect("Invalid JSON message");
                log::debug!("Parsed websocket message into: {:?}", result);

                match result {
                    WsSessionMessage::CreateGame(client_id) => self.create_game(client_id, ctx),
                    WsSessionMessage::JoinGame(ids) => self.join_game(ids, ctx),
                    WsSessionMessage::LeaveGame(ids) => self.leave_game(ids, ctx),
                    WsSessionMessage::Movement(ids, movement) => {
                        self.make_movement(ids, movement, ctx)
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

type ClientUUID = Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientAndGameId {
    game_id: Uuid,
    client_id: Uuid,
}

/// Represents a message sent by the client to the server.
#[derive(Debug, Deserialize, Serialize)]
enum WsSessionMessage {
    CreateGame(ClientUUID),
    JoinGame(ClientAndGameId),
    LeaveGame(ClientAndGameId),
    Movement(ClientAndGameId, BoardMovement),
}
