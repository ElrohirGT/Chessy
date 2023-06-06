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
    let piece_in_dest = board.get_piece(&destination);
    if let Some(p) = piece_in_dest {
        if p.color() == piece.color() {
            return Err(MovementError::DestinationCellOccupied);
        }
    }

    if let Some(check_positions) = board.get_check_positions() {
        if check_positions.contains(&destination) {
            return Err(MovementError::MovementDoesntRemoveCheck);
        }

        let valid_paths = get_movement_paths(&piece, &board);
    }
    todo!();
}
