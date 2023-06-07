use crate::{BoardPath, ChessPiece};

use super::{BoardPosition, ChessCell, PieceColors};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckedState {
    pub color_in_check: PieceColors,
    pub check_paths: Vec<BoardPath>,
}

#[derive(Debug, Serialize, Deserialize)]
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

    /// Moves a piece in the board itself. This method expects everything passed to it to be
    /// correct, so it doesn't checks for collisions nor movement patterns.
    pub(crate) fn move_piece(self, mut piece: ChessPiece, destination: BoardPosition) -> Board {
        piece.update_position(destination.clone());
        let Board {
            mut black_pieces,
            mut white_pieces,
            check_state,
            mut cells,
        } = self;
        let (dest_row, dest_column) = destination.into();

        let cell = cells
            .get_mut(dest_row)
            .unwrap()
            .get_mut(dest_column)
            .unwrap();

        *cell = match cell.piece() {
            Some(piece_in_cell) => match piece_in_cell.color() {
                PieceColors::Black => {
                    let index = black_pieces
                        .iter()
                        .position(|p| p == &piece_in_cell)
                        .expect(
                            format!("Theres no black piece that matches {:?}", &piece_in_cell)
                                .as_str(),
                        );
                    black_pieces.remove(index);

                    ChessCell::some(piece)
                }
                PieceColors::White => {
                    let index = white_pieces
                        .iter()
                        .position(|p| p == &piece_in_cell)
                        .expect(
                            format!("Theres no white piece that matches {:?}", &piece_in_cell)
                                .as_str(),
                        );
                    white_pieces.remove(index);

                    ChessCell::some(piece)
                }
            },
            None => ChessCell::some(piece),
        };

        Board::new(cells, black_pieces, white_pieces, check_state)
    }
}
