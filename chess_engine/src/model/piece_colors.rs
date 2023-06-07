use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceColors {
    Black,
    White,
}

impl std::fmt::Display for PieceColors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            PieceColors::Black => write!(f, "Black"),
            PieceColors::White => write!(f, "White"),
        }
    }
}

impl PieceColors {
    pub fn opponent(&self) -> PieceColors {
        match &self {
            PieceColors::Black => PieceColors::White,
            PieceColors::White => PieceColors::Black,
        }
    }
}
