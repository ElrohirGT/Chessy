use super::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BoardPath(pub Vec<BoardPosition>);

impl From<Vec<BoardPosition>> for BoardPath {
    fn from(value: Vec<BoardPosition>) -> Self {
        BoardPath(value)
    }
}
