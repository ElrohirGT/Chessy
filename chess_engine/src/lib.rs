mod model;
use self::model::*;

pub struct Game {
    pieces: Vec<ChessPiece>,
    board: Board,
    white_player: Player,
    black_player: Player,

    white_king: BoardPosition,
    black_king: BoardPosition,
}
