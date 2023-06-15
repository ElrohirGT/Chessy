use crate::{BoardPosition, ChessPiece};

/// Groups all the necessary information to move a piece.
pub struct BoardMovement {
    pub piece: ChessPiece,
    pub destination: BoardPosition,
}
