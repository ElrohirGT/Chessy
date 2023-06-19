use super::*;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub enum MovementSuccess{
    NormalMovement,
    CheckmateMovement,
    StalemateMovement,
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
    #[error("The king can't castle because the rook already moved!")]
    CantCastleBecauseTheRookMoved,
}

pub fn move_piece(
    movement: BoardMovement,
    board: &mut Board,
) -> Result<MovementSuccess, MovementError> {
    let BoardMovement { piece, destination } = movement;
    let piece_color = piece.color().clone();

    let piece_in_dest = board.get_piece(&destination);
    let destination_has_piece = piece_in_dest.is_some();
    let both_same_color = if let Some(p) = piece_in_dest {
        p.color() == &piece_color
    } else {
        false
    };

    let is_king = if let PieceTypes::King = piece.kind() {
        true
    } else {
        false
    };

    if destination_has_piece && both_same_color {
        return Err(MovementError::DestinationCellOccupied);
    }

    if let Some(check_positions) = board.get_check_positions() {
        return match (is_king, check_positions.contains(&destination)) {
            (true, true) => Err(MovementError::MovementWouldCauseCheck),
            (false, false) => Err(MovementError::MovementDoesntRemoveCheck),
            // The destination is a valid piece destination according to the piece type.
            // King: The destination is a position where he's not in check.
            // Any other piece: The destination blocks a check path.
            (is_king, _) => inner_move_piece(is_king, piece, destination, board),
        };
    } else {
        inner_move_piece(is_king, piece, destination, board)
    }
}

fn inner_move_piece(
    is_king: bool,
    piece: ChessPiece,
    destination: BoardPosition,
    board: &mut Board,
) -> Result<MovementSuccess, MovementError> {
    let piece_color = piece.color().clone();
    let opponent_piece_color = piece_color.opponent();

    let movement_pattern = board.get_movement_paths(&piece);
    let movement_positions: Vec<BoardPosition> = movement_pattern
        .into_iter()
        .map(|path| path.0)
        .flatten()
        .collect();

    let castling_state = board.get_castling_state(&piece_color);
    let mut castle_direction = None;
    if let (
        true,
        Some(CastlingState {
            can_use_left_rook,
            can_use_right_rook,
        }),
    ) = (is_king, castling_state)
    {
        let is_on_starting_positions = is_king_on_starting_position(&piece);
        if is_on_starting_positions {
            let wants_to_castle_left =
                check_castle_left(&piece.position().try_into().unwrap(), &destination);
            let wants_to_castle_right =
                check_castle_right(&piece.position().try_into().unwrap(), &destination);

            match (wants_to_castle_left, can_use_left_rook) {
                (true, true) => castle_direction = Some(ChessBoardDirections::Left),
                (true, false) => return Err(MovementError::CantCastleBecauseTheRookMoved),
                _ => {}
            }

            match (wants_to_castle_right, can_use_right_rook) {
                (true, true) => castle_direction = Some(ChessBoardDirections::Right),
                (true, false) => return Err(MovementError::CantCastleBecauseTheRookMoved),
                _ => {}
            }
        }
    } else if !movement_positions.contains(&destination) {
        return Err(MovementError::DestinationDoesntFollowMovementPattern);
    }

    board.move_piece(piece, &destination, castle_direction);

    let king_position = if is_king {
        destination
    } else {
        board.get_king_position(&piece_color)
    };

    if board.position_in_check(&king_position, &piece_color) {
        Err(MovementError::MovementDoesntRemoveCheck)
    } else if board.is_checkmate(&opponent_piece_color) {
        Ok(MovementSuccess::CheckmateMovement)
    } else if board.is_stalemate(&opponent_piece_color) {
        Ok(MovementSuccess::StalemateMovement)
    } else {
        Ok(MovementSuccess::NormalMovement)
    }
}
