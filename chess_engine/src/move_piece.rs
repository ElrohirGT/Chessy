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
    board: Board,
) -> Result<MovementSuccess, MovementError> {
    let piece_color = piece.color().clone();
    let opponent_piece_color = piece_color.opponent();

    let movement_pattern = get_movement_pattern(&piece);
    let movement_positions: Vec<BoardPosition> = movement_pattern
        .into_iter()
        .map(|path| path.0)
        .flatten()
        .collect();

    if !movement_positions.contains(&destination) {
        return Err(MovementError::DestinationDoesntFollowMovementPattern);
    }

    let board = board.move_piece(piece, &destination);

    let king_position = if is_king {
        destination
    } else {
        board.get_king_position(&piece_color)
    };

    if board.position_in_check(&king_position, &piece_color) {
        Err(MovementError::MovementDoesntRemoveCheck)
    } else if board.is_checkmate(&opponent_piece_color) {
        Ok(MovementSuccess::CheckmateMovement(board))
    } else if board.is_stalemate(&opponent_piece_color) {
        Ok(MovementSuccess::StalemateMovement(board))
    } else {
        Ok(MovementSuccess::NormalMovement(board))
    }
}
