use super::{BoardPosition, PieceColors, PieceTypes};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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

    /// Retrieves the piece position in the form of a tuple of array indeces.
    pub fn position(&self) -> (usize, usize) {
        (&self)
            .position
            .clone()
            .try_into()
            .expect("Coudln't convert the position to (usize, usize)")
    }

    /// Updates the inner position of this piece.
    pub(crate) fn update_position(&mut self, destination: BoardPosition) {
        (*self).position = destination;
    }
}
