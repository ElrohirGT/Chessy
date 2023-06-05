use super::*;

/// Get's all the valid positions this piece can move on the given board.
pub fn get_valid_movements_positions(piece: &ChessPiece, board: &Board) -> Vec<BoardPosition> {
    let mut pattern = get_movement_pattern(piece);
    let pawn_capture_pattern: Option<Vec<BoardPath>> = get_capture_pattern(piece, board);
    if let (PieceTypes::Pawn, Some(mut paths)) = (piece.kind(), pawn_capture_pattern) {
        pattern.append(&mut paths);
    }

    let possible_paths: Vec<BoardPath> = pattern
        .into_iter()
        .map(|path| {
            let path: Vec<BoardPosition> = path.0;
            path.into_iter()
                .scan(false, |found_piece, position| {
                    if *found_piece {
                        return None;
                    }

                    let piece_option = get_piece(position.clone(), board);
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
        .collect();
    possible_paths
        .into_iter()
        .map(|path| path.0)
        .flatten()
        .collect()
}

fn get_capture_pattern(piece: &ChessPiece, board: &Board) -> Option<Vec<BoardPath>> {
    let oponent_color = piece.color().opponent();
    if let PieceTypes::Pawn = piece.kind() {
        let (row, column) = piece.position();
        let paths: Vec<BoardPath> = vec![(1, -1), (1, 1)]
            .into_iter()
            .filter_map(|(r, c): (isize, isize)| {
                get_piece(
                    (row as isize + r, column as isize + c).try_into().ok()?,
                    board,
                )
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

/// Tries to get a piece on the given position. There may not be a piece on that position so it
/// returns an option.
pub fn get_piece(position: BoardPosition, board: &Board) -> Option<ChessPiece> {
    let cell = board.get_cell(position);
    cell.piece()
}

/// Get's all the possible movements, valid or invalid that a given piece can make.
/// The return value is a collections of paths the given piece can make. This makes easier for
/// checking for collisions down the line.
pub fn get_movement_pattern(piece: &ChessPiece) -> Vec<BoardPath> {
    let kind = *piece.kind();
    let (row, column) = piece.position();
    match kind {
        PieceTypes::Pawn => vec![BoardPath(vec![(row + 1, column).try_into().unwrap()])],
        PieceTypes::Rook => rook_movement_pattern(row, column),
        PieceTypes::Knight => pattern_from_vec(
            vec![
                (2, 1),
                (1, 2),
                (2, -1),
                (1, -2),
                (-1, -2),
                (-2, -1),
                (-2, 1),
                (-1, 2),
            ],
            row,
            column,
        ),
        PieceTypes::Bishop => bishop_movement_pattern(row, column),
        PieceTypes::Queen => {
            let mut r_pattern = rook_movement_pattern(row, column);
            let mut b_pattern = bishop_movement_pattern(row, column);

            r_pattern.append(&mut b_pattern);
            r_pattern
        }
        PieceTypes::King => pattern_from_vec(
            vec![
                (1, -1),
                (1, 0),
                (1, 1),
                (0, -1),
                (0, 1),
                (-1, -1),
                (-1, 1),
                (-1, 0),
            ],
            row,
            column,
        ),
    }
}

fn pattern_from_vec(vec: Vec<(isize, isize)>, row: usize, column: usize) -> Vec<BoardPath> {
    vec.into_iter()
        .map(|(r, c): (isize, isize)| (row as isize + r, column as isize + c))
        .filter_map(|p| p.try_into().ok())
        .map(|p: BoardPosition| vec![p])
        .map(BoardPath::from)
        .collect()
}

fn rook_movement_pattern(row: usize, column: usize) -> Vec<BoardPath> {
    (0..4)
        .map(|i| match i {
            1 => (1..(7 - row))
                .filter_map(|i| (row + i, column).try_into().ok())
                .collect(),
            2 => (1..(7 - column))
                .filter_map(|i| (row, column + i).try_into().ok())
                .collect(),
            3 => (1..row)
                .filter_map(|i| (row - i, column).try_into().ok())
                .collect(),
            4 => (1..column)
                .filter_map(|i| (row, column - i).try_into().ok())
                .collect(),
            _ => unreachable!(),
        })
        .map(|v: Vec<BoardPosition>| BoardPath(v))
        .collect()
}

fn bishop_movement_pattern(row: usize, column: usize) -> Vec<BoardPath> {
    vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
        .into_iter()
        .map(|(r, c): (isize, isize)| {
            (0..7)
                .filter_map(|i| {
                    (row as isize + i * r, column as isize + i * c)
                        .try_into()
                        .ok()
                })
                .collect::<Vec<BoardPosition>>()
        })
        .map(BoardPath::from)
        .collect()
}
