use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardPath(pub Vec<BoardPosition>);

impl From<Vec<BoardPosition>> for BoardPath {
    fn from(value: Vec<BoardPosition>) -> Self {
        BoardPath(value)
    }
}
