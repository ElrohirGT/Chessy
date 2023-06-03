use super::*;

#[derive(Debug)]
pub struct BoardPath(pub(crate) Vec<BoardPosition>);
impl From<Vec<BoardPosition>> for BoardPath {
    fn from(value: Vec<BoardPosition>) -> Self {
        BoardPath(value)
    }
}

impl From<BoardPath> for Vec<BoardPosition> {
    fn from(value: BoardPath) -> Self {
        value.0
    }
}
