use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PieceTypes {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
