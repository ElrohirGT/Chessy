use thiserror::Error;

mod board;
mod board_path;
mod board_position;
mod chess_board_directions;
mod chess_cell;
mod chess_file;
mod chess_piece;
mod chess_rank;
mod game_movement;
mod piece_colors;
mod piece_types;
mod player;

pub use self::board::*;
pub use self::board_path::*;
pub use self::board_position::*;
pub use self::chess_board_directions::*;
pub use self::chess_cell::*;
pub use self::chess_file::*;
pub use self::chess_piece::*;
pub use self::chess_rank::*;
pub use self::game_movement::*;
pub use self::piece_colors::*;
pub use self::piece_colors::*;
pub use self::piece_types::*;
pub use self::player::*;

#[derive(Debug, Error)]
pub enum FromArrayIndexError {
    #[error("The array index must be equal or less than {0}")]
    IndexTooBig(usize),
}
pub trait ArrayIndex: Sized {
    fn to_index(&self) -> usize;
    fn from_index(value: usize) -> Result<Self, FromArrayIndexError>;
}

#[derive(Debug, Error)]
pub enum FromMatrixPositionError {
    #[error("The matrix row must be equal or less than {0}")]
    MatrixRowTooBig(usize),
    #[error("The matrix column must be equal or less than {0}")]
    MatrixColumnTooBig(usize),
}
pub trait MatrixPosition: Sized {
    fn to_matrix_position(&self) -> (usize, usize);
    fn from_matrix_position(value: (usize, usize)) -> Result<Self, FromMatrixPositionError>;
}
