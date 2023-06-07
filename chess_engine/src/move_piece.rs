use super::*;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub enum MovementSuccess {
    NormalMovement(Board),
    CheckmateMovement(Board),
    StalemateMovement(Board),
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
    let piece_color = piece.color().clone();
    let opponent_piece_color = piece.color().opponent();

    let piece_in_dest = board.get_piece(&destination);
    let destination_has_piece = piece_in_dest.is_some();
    let both_same_color = if let Some(p) = piece_in_dest {
        p.color() == &piece_color
    } else {
        false
    };

    if destination_has_piece && both_same_color {
        return Err(MovementError::DestinationCellOccupied);
    }

    if let Some(check_positions) = board.get_check_positions() {
        let is_king = if let PieceTypes::King = piece.kind() {
            true
        } else {
            false
        };

        return match (is_king, check_positions.contains(&destination)) {
            (true, true) => Err(MovementError::MovementWouldCauseCheck),
            (false, false) => Err(MovementError::MovementDoesntRemoveCheck),
            // The piece is King and the destination position is not in the current check
            // positions.
            (true, false) => {
                let movement_pattern = get_movement_pattern(&piece);
                let movement_positions: Vec<BoardPosition> = movement_pattern
                    .into_iter()
                    .map(|path| path.0)
                    .flatten()
                    .collect();

                if !movement_positions.contains(&destination) {
                    return Err(MovementError::DestinationDoesntFollowMovementPattern);
                }

                let board_with_movement: Board = board.move_piece(piece, destination.clone());
                if position_in_check(&destination, &piece_color, &board_with_movement) {
                    Err(MovementError::MovementDoesntRemoveCheck)
                } else if is_stalemate(opponent_piece_color, &board_with_movement) {
                    Ok(MovementSuccess::StalemateMovement(board_with_movement))
                } else {
                    Ok(MovementSuccess::NormalMovement(board_with_movement))
                }
            }
            // The piece is not a King and the destination position is in the current check
            // positions.
            (false, true) => todo!(),
        };
    }
    todo!();
}

fn is_stalemate(opponent: PieceColors, board_with_movement: &Board) -> bool {
    todo!()
}
