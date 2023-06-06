use chess_engine::{get_starting_board, move_piece, ChessPiece, PieceColors, PieceTypes};
use serde_json;

fn main() {
    let board = get_starting_board();

    let respone = move_piece(
        ChessPiece::new(
            PieceTypes::Pawn,
            "e1".try_into().unwrap(),
            PieceColors::White,
        ),
        "e2".try_into().unwrap(),
        board,
    );

    println!("{}", serde_json::to_string(&respone.unwrap()).unwrap());
}
