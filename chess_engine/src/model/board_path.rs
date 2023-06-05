use super::*;

#[derive(Debug)]
pub struct BoardPath(pub Vec<BoardPosition>);

impl BoardPath {
    pub(crate) fn new(positions: Vec<BoardPosition>) -> Self {
        BoardPath(positions)
    }
}

impl From<Vec<BoardPosition>> for BoardPath {
    fn from(value: Vec<BoardPosition>) -> Self {
        BoardPath(value)
    }
}
