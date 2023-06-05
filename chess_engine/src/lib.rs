mod game_model;
mod get_starting_board;
mod model;
mod valid_movements_positions;

pub use self::game_model::*;
pub use self::get_starting_board::*;
pub use self::model::*;
pub use self::valid_movements_positions::*;

pub fn create_game(config: GameConfig) -> Game {
    Game::new(config)
}
