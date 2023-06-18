use std::{collections::HashMap, sync::Arc};

use actix::Recipient;
use chess_engine::{get_starting_board, Board, PieceColors};
use serde::Serialize;
use uuid::Uuid;

use crate::{player::Player, websocket::GameMessage, AppState};

type Client = Recipient<GameMessage>;

/// Represents a chess game in the server
pub struct ServerGame {
    pub(crate) game: Game,
    pub(crate) sessions: HashMap<Uuid, Client>,
}

impl ServerGame {
    pub fn is_full(&self) -> bool {
        self.sessions.len() >= 2
    }

    pub fn add_opponent(
        &mut self,
        client_id: Uuid,
        client: Recipient<GameMessage>,
        users: &mut Arc<AppState>,
    ) {
        self.sessions.insert(client_id, client);
        self.game.add_opponent(client_id, users);
    }
}

/// Represents a Chess Game.
#[derive(Debug, Serialize)]
pub struct Game {
    players: HashMap<PieceColors, Player>,
    board: Board,
    initial_ms_per_player: u64,
}

/// Configuration to start a Chess Game.
#[derive(Debug)]
pub struct GameConfig {
    players_names: HashMap<PieceColors, (Uuid, Arc<str>)>,
    ms_per_player: u64,
    board: Board,
}

impl GameConfig {
    pub fn new(players_names: HashMap<PieceColors, (Uuid, Arc<str>)>, ms_per_player: u64) -> Self {
        let board = get_starting_board();
        GameConfig {
            players_names,
            ms_per_player,
            board,
        }
    }

    pub fn new_with_board(
        players_names: HashMap<PieceColors, (Uuid, Arc<str>)>,
        ms_per_player: u64,
        board: Board,
    ) -> Self {
        GameConfig {
            players_names,
            ms_per_player,
            board,
        }
    }
}

impl Game {
    pub fn new(
        GameConfig {
            players_names,
            ms_per_player,
            board,
        }: GameConfig,
    ) -> Self {
        let players = players_names
            .into_iter()
            .map(|(color, (id, name))| (color.clone(), Player::new(id, name, color, ms_per_player)))
            .collect();

        let initial_ms_per_player = ms_per_player;

        Game {
            players,
            board,
            initial_ms_per_player,
        }
    }

    pub fn add_opponent(&mut self, client_id: Uuid, users: &mut Arc<AppState>) {
        let (color, _) = self
            .players
            .iter()
            .next()
            .expect("No players found in the game to add an opponent!");
        let color = color.opponent();

        self.add_player(client_id, users, color);
    }

    fn add_player(&mut self, client_id: Uuid, users: &mut Arc<AppState>, color: PieceColors) {
        let name = get_name(&client_id, users);
        let player = Player::new(
            client_id.clone(),
            name,
            color.clone(),
            self.initial_ms_per_player,
        );
        self.players.insert(color, player);
    }

    pub(crate) fn remove_player(&mut self, client_id: &Uuid) {
        let player_color = self.players.iter_mut().find_map(|(color, player)| {
            if player.id() == client_id {
                Some(color.clone())
            } else {
                None
            }
        });

        if let Some(color) = player_color {
            self.players.remove(&color);
        }
    }
}

pub(crate) fn get_name(client_id: &Uuid, users: &mut Arc<AppState>) -> Arc<str> {
    let users = users
        .users
        .lock()
        .expect(r#"Couldn't aquire the lock to users or it has been poisoned!"#);

    let name = users
        .get(client_id)
        .expect(format!("No username found with id {}", client_id).as_str());

    Arc::clone(name)
}
