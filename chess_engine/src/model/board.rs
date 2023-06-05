use crate::BoardPath;

use super::{BoardPosition, ChessCell, PieceColors};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CheckedState {
    pub color_in_check: PieceColors,
    pub check_paths: Vec<BoardPath>,
}

#[derive(Debug, Serialize)]
pub struct Board {
    pub check_state: Option<CheckedState>,
    pub cells: Vec<Vec<ChessCell>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<ChessCell>>, check_state: Option<CheckedState>) -> Self {
        Board { cells, check_state }
    }

    pub fn color_in_check(&self, owner: &PieceColors) -> bool {
        if let Some(CheckedState { color_in_check, .. }) = &self.check_state {
            color_in_check == owner
        } else {
            false
        }
    }

    pub fn get_cell(&self, position: BoardPosition) -> &ChessCell {
        let (row, column): (usize, usize) = position.into();
        (&self).cells.get(row).unwrap().get(column).unwrap()
    }
}
