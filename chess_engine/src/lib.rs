mod game_model;
mod model;

use thiserror::Error;

pub use self::game_model::*;
pub use self::model::*;

pub fn create_game(config: GameConfig) -> Game {
    Game::new(config)
}

#[derive(Debug, Error)]
pub enum PieceMovementError {
    #[error("No chess piece was found on position {0}.")]
    NoPieceToMove(BoardPosition),
    #[error("The destination cell ({0}) is occupied by a foe piece.")]
    DestinationCellOccupied(BoardPosition),
    #[error("The king of the player ({0}) continues to be in check.")]
    PlayerKingStillInCheck(PieceColors),
}

pub fn move_piece(
    from: BoardPosition,
    to: BoardPosition,
    board: Board,
) -> Result<Board, PieceMovementError> {
    let movement_piece =
        get_piece(&board, &from).ok_or(PieceMovementError::NoPieceToMove(from.clone()))?;
    let piece_color = movement_piece.owner;
    let destination_cell = get_cell(&board, &to);

    if destination_cell.piece_has_color(&piece_color) {
        return Err(PieceMovementError::DestinationCellOccupied(to.clone()));
    }

    let board_after = inner_move_piece(from, to, &board)?;
    if board.color_in_check(&piece_color) && board_after.color_in_check(&piece_color) {
        return Err(PieceMovementError::PlayerKingStillInCheck(
            piece_color.clone(),
        ));
    }

    todo!()
}

pub fn inner_move_piece(
    from: BoardPosition,
    to: BoardPosition,
    board: &Board,
) -> Result<Board, PieceMovementError> {
    todo!()
}

pub fn get_cell<'a>(board: &'a Board, position: &'a BoardPosition) -> &'a ChessCell {
    let row = position.row.to_index();
    let column = position.column.to_index();
    board.cells.get(row).unwrap().get(column).unwrap()
}

pub fn get_piece(board: &Board, position: &BoardPosition) -> Option<ChessPiece> {
    todo!()
}
