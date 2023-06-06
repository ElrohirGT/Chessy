use crate::{BoardPath, ChessPiece};

use super::{BoardPosition, ChessCell, PieceColors};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CheckedState {
    pub color_in_check: PieceColors,
    pub check_paths: Vec<BoardPath>,
}

#[derive(Debug, Serialize)]
pub struct Board {
    pub black_pieces: Vec<ChessPiece>,
    pub white_pieces: Vec<ChessPiece>,
    pub check_state: Option<CheckedState>,
    pub cells: Vec<Vec<ChessCell>>,
}

impl Board {
    pub fn new(
        cells: Vec<Vec<ChessCell>>,
        black_pieces: Vec<ChessPiece>,
        white_pieces: Vec<ChessPiece>,
        check_state: Option<CheckedState>,
    ) -> Self {
        Board {
            cells,
            black_pieces,
            white_pieces,
            check_state,
        }
    }

    pub fn is_in_check(&self, owner: &PieceColors) -> bool {
        if let Some(CheckedState { color_in_check, .. }) = &self.check_state {
            color_in_check == owner
        } else {
            false
        }
    }

    pub fn get_cell(&self, position: &BoardPosition) -> &ChessCell {
        let (row, column): (usize, usize) = position.into();
        (&self).cells.get(row).unwrap().get(column).unwrap()
    }

    /// Tries to get a piece on the given position. There may not be a piece on that position so it
    /// returns an option.
    pub fn get_piece(&self, position: &BoardPosition) -> Option<ChessPiece> {
        (&self).get_cell(position).piece()
    }

    pub fn get_check_positions(&self) -> Option<Vec<BoardPosition>> {
        match &self.check_state {
            Some(CheckedState { check_paths, .. }) => Some(
                check_paths
                    .iter()
                    .map(|pos: &BoardPath| pos.clone().0)
                    .flatten()
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn get_pieces_from(&self, opponent: PieceColors) -> &Vec<ChessPiece> {
        match opponent {
            PieceColors::Black => &self.white_pieces,
            PieceColors::White => &self.black_pieces,
        }
    }
}
