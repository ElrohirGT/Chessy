mod game_model;
mod get_starting_board;
mod model;
mod move_piece;
mod valid_movements_positions;

pub use self::game_model::*;
pub use self::get_starting_board::*;
pub use self::model::*;
pub use self::move_piece::*;
pub use self::valid_movements_positions::*;

pub fn create_game(config: GameConfig) -> Game {
    Game::new(config)
}
