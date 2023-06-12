use crate::{
    get_movement_pattern, get_valid_movements_positions, BoardPath, ChessPiece, PieceTypes,
};

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

    pub white_king_position: BoardPosition,
    pub black_king_position: BoardPosition,

    pub check_state: Option<CheckedState>,
    pub cells: Vec<Vec<ChessCell>>,
}

impl Board {
    pub fn new(
        cells: Vec<Vec<ChessCell>>,
        black_pieces: Vec<ChessPiece>,
        white_pieces: Vec<ChessPiece>,
        white_king_position: BoardPosition,
        black_king_position: BoardPosition,
        check_state: Option<CheckedState>,
    ) -> Self {
        Board {
            cells,
            black_pieces,
            white_pieces,
            check_state,
            white_king_position,
            black_king_position,
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
        self.cells.get(row).unwrap().get(column).unwrap()
    }

    /// Tries to get a piece on the given position. There may not be a piece on that position so it
    /// returns an option.
    pub fn get_piece(&self, position: &BoardPosition) -> Option<ChessPiece> {
        self.get_cell(position).piece()
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

    fn get_pieces_from(&self, color: &PieceColors) -> &Vec<ChessPiece> {
        match color {
            PieceColors::Black => &self.black_pieces,
            PieceColors::White => &self.white_pieces,
        }
    }

    /// Moves a piece in the board itself. This method expects everything passed to it to be
    /// correct, so it doesn't checks for collisions nor movement patterns.
    pub(crate) fn move_piece(self, mut piece: ChessPiece, destination: &BoardPosition) -> Board {
        piece.update_position(destination.clone());
        let Board {
            mut black_pieces,
            mut white_pieces,
            check_state,
            mut cells,
            mut white_king_position,
            mut black_king_position,
        } = self;

        match (piece.kind(), piece.color()) {
            (PieceTypes::King, PieceColors::Black) => black_king_position = destination.clone(),
            (PieceTypes::King, PieceColors::White) => white_king_position = destination.clone(),
            _ => {}
        }

        let (dest_row, dest_column) = destination.into();
        let piece_color = piece.color().clone();

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
                            format!("There's no black piece that matches {:?}", &piece_in_cell)
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
                            format!("There's no white piece that matches {:?}", &piece_in_cell)
                                .as_str(),
                        );
                    white_pieces.remove(index);

                    ChessCell::some(piece)
                }
            },
            None => ChessCell::some(piece),
        };

        let mut board = Board::new(
            cells,
            black_pieces,
            white_pieces,
            white_king_position,
            black_king_position,
            check_state,
        );

        board.update_check_state(&piece_color);

        board
    }

    /// Get's the king position of the given color.
    pub fn get_king_position(&self, piece_color: &PieceColors) -> BoardPosition {
        match piece_color {
            PieceColors::Black => self.black_king_position.clone(),
            PieceColors::White => self.white_king_position.clone(),
        }
    }

    /// Checks if a king of the given color in the given position would be in check in the given board.
    /// Returns true if the positions would be in check.
    pub fn position_in_check(&self, position: &BoardPosition, color: &PieceColors) -> bool {
        let opponent_pieces = self.get_pieces_from(&color.opponent());
        opponent_pieces
            .iter()
            .map(|piece| self.get_movement_paths(piece))
            .flatten()
            .map(|path: BoardPath| path.0)
            .flatten()
            .any(|pos| pos == *position)
    }

    /// Get's all the valid movement paths the piece can take, considering collisions with other pieces.
    /// This method doesn't considers the check state.
    pub(crate) fn get_movement_paths(&self, piece: &ChessPiece) -> Vec<BoardPath> {
        let mut pattern = get_movement_pattern(piece);
        let pawn_capture_pattern: Option<Vec<BoardPath>> = self.get_capture_pattern(piece);

        if let (PieceTypes::Pawn, Some(mut paths)) = (piece.kind(), pawn_capture_pattern) {
            pattern.append(&mut paths);
        }

        let paths: Vec<BoardPath> = self.colission_detection(pattern, piece);
        paths
    }

    /// Get's the movement pattern of this piece if it can capture a piece.
    /// The only piece that actually needs this method is the pawn, because it can capture diagonally.
    pub(crate) fn get_capture_pattern(&self, piece: &ChessPiece) -> Option<Vec<BoardPath>> {
        let oponent_color = piece.color().opponent();
        if let PieceTypes::Pawn = piece.kind() {
            let (row, column) = piece.position();
            let positions_to_check = match piece.color() {
                PieceColors::Black => vec![(-1, -1), (-1, 1)],
                PieceColors::White => vec![(1, -1), (1, 1)],
            };

            let paths: Vec<BoardPath> = positions_to_check
                .into_iter()
                .filter_map(|(r, c): (isize, isize)| {
                    self.get_piece(&((row as isize + r, column as isize + c).try_into().ok()?))
                })
                .filter(|chess_piece| *chess_piece.color() == oponent_color)
                .filter_map(|chess_piece| chess_piece.position().try_into().ok())
                .map(|p: BoardPosition| vec![p])
                .map(BoardPath::from)
                .collect();
            Some(paths)
        } else {
            None
        }
    }

    /// Checks all the paths and checks for collisions with another pieces.
    /// The inputs are the piece to and the pattern of the given piece.
    /// Returns the new paths now taking into account collision with other pieces.
    pub(crate) fn colission_detection(
        &self,
        pattern: Vec<BoardPath>,
        piece: &ChessPiece,
    ) -> Vec<BoardPath> {
        pattern
            .into_iter()
            .map(|path| {
                let path: Vec<BoardPosition> = path.0;
                path.into_iter()
                    .scan(false, |found_piece, position| {
                        if *found_piece {
                            return None;
                        }

                        let piece_option = self.get_piece(&position);
                        let has_enemy_piece = if let Some(cell_piece) = &piece_option {
                            *cell_piece.color() == piece.color().opponent()
                        } else {
                            false
                        };

                        *found_piece = piece_option.is_some();

                        if piece_option.is_none() || has_enemy_piece {
                            Some(position)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<BoardPosition>>()
            })
            .map(BoardPath::from)
            .collect()
    }

    /// Checks if the player of the given color is in stalemate.
    pub fn is_stalemate(&self, color: &PieceColors) -> bool {
        match &self.check_state {
            Some(CheckedState { color_in_check, .. }) => {
                if color_in_check == color {
                    false
                } else {
                    self.can_any_piece_move(color)
                }
            }
            None => self.can_any_piece_move(color),
        }
    }

    fn can_any_piece_move(&self, color: &PieceColors) -> bool {
        let pieces = self.get_pieces_from(color);
        pieces
            .iter()
            .map(|p| get_valid_movements_positions(&p, self))
            .any(|mp| !mp.is_empty())
    }

    fn update_check_state(&mut self, color_to_check_first: &PieceColors) {
        let opponent_color = color_to_check_first.opponent();
        let colors = vec![color_to_check_first, &opponent_color];

        let mut new_check_state = None;
        for color in colors {
            if let Some(_) = new_check_state {
                break;
            }
            let king_position = self.get_king_position(color);
            let opponent_pieces = self.get_pieces_from(&color.opponent());

            let check_paths: Vec<BoardPath> = opponent_pieces
                .iter()
                .map(|p| self.get_movement_paths(p))
                .flatten()
                .filter(|movement_path| movement_path.0.contains(&king_position))
                .collect();

            if !check_paths.is_empty() {
                new_check_state = Some(CheckedState {
                    color_in_check: color.clone(),
                    check_paths,
                });
            }
        }

        self.check_state = new_check_state;
    }

    /// Checks if the king of the given color is in checkmate.
    pub fn is_checkmate(&self, king_color: &PieceColors) -> bool {
        let king_position = self.get_king_position(king_color);
        let king = self
            .get_piece(&king_position)
            .expect("The king position should always be synchronized.");

        match &self.check_state {
            Some(CheckedState {
                color_in_check,
                check_paths,
            }) => {
                if color_in_check != king_color {
                    false
                } else {
                    let can_king_move = get_valid_movements_positions(&king, self).is_empty();
                    if can_king_move {
                        return false;
                    }
                    if check_paths.len() >= 1 {
                        return true;
                    }

                    let check_positions: Vec<BoardPosition> = check_paths
                        .iter()
                        .map(|pos| pos.clone().0)
                        .flatten()
                        .collect();

                    self.get_pieces_from(king_color)
                        .into_iter()
                        .map(|p| self.get_movement_paths(p))
                        .flatten()
                        .map(|path| path.0)
                        .flatten()
                        .any(|pos| check_positions.contains(&pos))
                }
            }
            None => false,
        }
    }
}
