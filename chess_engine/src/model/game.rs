use crate::get_starting_board;

use super::*;

/// Represents a Chess Game.
pub struct Game {
    white_player: Player,
    black_player: Player,

    board: Board,
}

/// Configuration to start a Chess Game.
pub struct GameConfig {
    white_player_name: String,
    black_player_name: String,
    ms_per_player: u64,
    board: Board,
}

impl GameConfig {
    pub fn new(white_player_name: String, black_player_name: String, ms_per_player: u64) -> Self {
        let board = get_starting_board();
        GameConfig {
            white_player_name,
            black_player_name,
            ms_per_player,
            board,
        }
    }

    pub fn new_with_board(
        white_player_name: String,
        black_player_name: String,
        ms_per_player: u64,
        board: Board,
    ) -> Self {
        GameConfig {
            white_player_name,
            black_player_name,
            ms_per_player,
            board,
        }
    }
}

impl Game {
    pub fn new(
        GameConfig {
            white_player_name,
            black_player_name,
            ms_per_player,
            board,
        }: GameConfig,
    ) -> Self {
        let white_player = Player::new(white_player_name, PieceColors::White, ms_per_player);
        let black_player = Player::new(black_player_name, PieceColors::Black, ms_per_player);

        Game {
            white_player,
            black_player,
            board,
        }
    }
}
