use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PieceTypes {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
