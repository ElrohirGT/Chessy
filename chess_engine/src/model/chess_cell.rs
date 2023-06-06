use super::{ChessPiece, PieceColors};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChessCell(pub Option<ChessPiece>);
impl ChessCell {
    pub fn some(piece: ChessPiece) -> ChessCell {
        ChessCell(Some(piece))
    }
    pub fn none() -> ChessCell {
        ChessCell(None)
    }
    pub fn is_occupied(&self) -> bool {
        self.0.is_some()
    }
    pub fn piece_has_color(&self, color: &PieceColors) -> bool {
        if let Some(piece) = &self.0 {
            piece.color() == color
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn piece(&self) -> Option<ChessPiece> {
        (&self).0.clone()
    }

    pub fn position(&self) -> Option<(usize, usize)> {
        (&self).0.clone().map(|piece| piece.position())
    }
}
