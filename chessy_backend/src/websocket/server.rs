use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use chess_engine::PieceColors;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use uuid::Uuid;

use crate::{
    game::{get_name, Game, GameConfig},
    player::Player,
    AppState,
};

use super::{CreateGame, GameMessage, JoinGame, JoinedGameResponses, LeaveGame, SendMovement};

pub struct ChessServer {
    games: HashMap<Uuid, Game>,
    rng: ThreadRng,
    users: Arc<AppState>,
}

impl ChessServer {
    pub fn new(users: Arc<AppState>) -> Self {
        ChessServer {
            games: HashMap::new(),
            users,
            rng: thread_rng(),
        }
    }
}

impl Actor for ChessServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveGame>(ctx);
        self.subscribe_system_async::<SendMovement>(ctx);
    }
}

impl Handler<CreateGame> for ChessServer {
    type Result = MessageResult<CreateGame>;

    fn handle(&mut self, msg: CreateGame, ctx: &mut Self::Context) -> Self::Result {
        let CreateGame { client_id, client } = msg;
        let ChessServer { games, rng, users } = self;

        let game_id = Uuid::new_v4();
        log::debug!(
            "Client ID (`{}`) started game (`{}`) creation...",
            client_id,
            game_id
        );
        let color = get_random_color(rng);
        let name = get_name(&client_id, users);
        log::debug!(
            "Client ID (`{}`) identified as `{}` and will play `{}`",
            client_id,
            name,
            color
        );

        let players = HashMap::from([(color, (client_id, name))]);
        let game_config = GameConfig::new(players, 10 * 60 * 1000);

        let sessions = HashMap::from([(client_id, client)]);
        let game = Game::new(game_config, sessions);
        log::debug!("Created game:\n{:?}", game);

        games.insert(game_id.clone(), game);

        MessageResult(game_id)
    }
}

fn get_random_color(rng: &mut ThreadRng) -> PieceColors {
    if rng.gen() {
        PieceColors::Black
    } else {
        PieceColors::White
    }
}

impl Handler<JoinGame> for ChessServer {
    type Result = MessageResult<JoinGame>;

    fn handle(&mut self, msg: JoinGame, ctx: &mut Self::Context) -> Self::Result {
        let JoinGame {
            game_id,
            client_id,
            client,
        } = msg;

        let ChessServer { games, rng, users } = self;
        match games.get_mut(&game_id) {
            Some(game) => {
                if game.is_full() {
                    MessageResult(JoinedGameResponses::GameFull)
                } else {
                    game.add_opponent(client_id, client, users);
                    MessageResult(JoinedGameResponses::JoinedGame)
                }
            }
            None => MessageResult(JoinedGameResponses::GameNotFound),
        }
    }
}

impl Handler<LeaveGame> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveGame, ctx: &mut Self::Context) -> Self::Result {
        let LeaveGame { game_id, client_id } = msg;
        if let Some(game) = self.games.get_mut(&game_id) {
            // TODO Remove player from game
        }
    }
}

impl Handler<SendMovement> for ChessServer {
    type Result = ();

    fn handle(&mut self, msg: SendMovement, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
