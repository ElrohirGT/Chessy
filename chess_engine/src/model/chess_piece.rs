use super::{BoardPosition, PieceColors, PieceTypes};

#[derive(Debug, Clone)]
pub struct ChessPiece {
    kind: PieceTypes,
    position: BoardPosition,
    owner: PieceColors,
}

impl ChessPiece {
    pub fn new(kind: PieceTypes, position: BoardPosition, owner: PieceColors) -> ChessPiece {
        ChessPiece {
            kind,
            position,
            owner,
        }
    }

    pub fn kind(&self) -> &PieceTypes {
        &self.kind
    }

    pub fn color(&self) -> &PieceColors {
        &self.owner
    }

    pub fn position(&self) -> (usize, usize) {
        (&self)
            .position
            .clone()
            .try_into()
            .expect("Coudln't convert the position to (usize, usize)")
    }
}
