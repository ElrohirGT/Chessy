use super::*;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub enum MovementSuccess {
    NormalMovement(Board),
    CheckmateMovement(Board),
    DrawMovement(Board),
}

#[derive(Debug, Error, Serialize)]
pub enum MovementError {
    #[error("The destination positions doesn't follow the movement pattern.")]
    DestinationDoesntFollowMovementPattern,
    #[error("The destination cell is already occupied by a foe piece.")]
    DestinationCellOccupied,
    #[error("The movement of this piece would cause a check to the king.")]
    MovementWouldCauseCheck,
    #[error("The king of this piece is in check!")]
    MovementDoesntRemoveCheck,
}

pub fn move_piece(
    piece: ChessPiece,
    destination: BoardPosition,
    board: Board,
) -> Result<MovementSuccess, MovementError> {
    Ok(MovementSuccess::NormalMovement(board))
}
