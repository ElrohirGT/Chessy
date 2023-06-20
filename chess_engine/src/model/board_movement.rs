use crate::{BoardPosition, ChessPiece};
use serde::{Deserialize, Serialize};

/// Groups all the necessary information to move a piece.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoardMovement {
    pub piece: ChessPiece,
    pub destination: BoardPosition,
}
