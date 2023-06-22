use std::{collections::HashMap, sync::Arc, time::Instant};

use actix::Recipient;
use chess_engine::{get_starting_board, Board, PieceColors};
use serde::Serialize;
use uuid::Uuid;

use crate::{player::Player, websocket::GameMessage};

type Client = Recipient<GameMessage>;

/// Represents a chess game in the server
pub struct ServerGame {
    pub(crate) game: Game,
    pub(crate) sessions: HashMap<Uuid, Client>,
    pub(crate) last_move: Instant,
}

impl ServerGame {
    pub fn is_full(&self) -> bool {
        self.sessions.len() >= 2
    }

    pub fn add_opponent(
        &mut self,
        client_id: Uuid,
        name: Arc<str>,
        client: Recipient<GameMessage>,
    ) -> Player {
        self.sessions.insert(client_id, client);
        self.game.add_opponent(client_id, name)
    }

    pub fn update_last_move(&mut self) {
        self.last_move = Instant::now();
    }

    pub fn new(game: Game, sessions: HashMap<Uuid, Recipient<GameMessage>>) -> Self {
        ServerGame {
            game,
            sessions,
            last_move: Instant::now(),
        }
    }
}

/// Represents a Chess Game.
#[derive(Debug, Serialize, Clone)]
pub struct Game {
    pub players: HashMap<PieceColors, Player>,
    pub board: Board,
    pub initial_ms_per_player: u128,
}

/// Configuration to start a Chess Game.
#[derive(Debug)]
pub struct GameConfig {
    players_names: HashMap<PieceColors, (Uuid, Arc<str>)>,
    ms_per_player: u128,
    board: Board,
}

impl GameConfig {
    pub fn new(players_names: HashMap<PieceColors, (Uuid, Arc<str>)>, ms_per_player: u128) -> Self {
        let board = get_starting_board();
        GameConfig {
            players_names,
            ms_per_player,
            board,
        }
    }

    pub fn new_with_board(
        players_names: HashMap<PieceColors, (Uuid, Arc<str>)>,
        ms_per_player: u128,
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

    pub fn add_opponent(&mut self, client_id: Uuid, name: Arc<str>) -> Player {
        let (color, _) = self
            .players
            .iter()
            .next()
            .expect("No players found in the game to add an opponent!");
        let color = color.opponent();

        self.add_player(client_id, name, color)
    }

    fn add_player(&mut self, client_id: Uuid, name: Arc<str>, color: PieceColors) -> Player {
        let player = Player::new(client_id, name, color.clone(), self.initial_ms_per_player);
        self.players.insert(color, player.clone());

        player
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
