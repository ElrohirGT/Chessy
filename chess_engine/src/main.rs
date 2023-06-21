use chess_engine::{
    get_starting_board, move_piece, BoardMovement, ChessPiece, PieceColors, PieceTypes,
};

fn main() {
    let mut board = get_starting_board();

    let piece = ChessPiece::new(
        PieceTypes::Pawn,
        "e2".try_into().unwrap(),
        PieceColors::White,
    );
    let destination = "e4".try_into().unwrap();
    let respone = move_piece(BoardMovement { destination, piece }, &mut board);

    println!("{}", serde_json::to_string(&respone.unwrap()).unwrap());
}
