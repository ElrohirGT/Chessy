mod game_model;
mod get_starting_board;
mod model;
mod move_piece;
mod valid_movements_positions;

pub use self::game_model::*;
pub use self::get_starting_board::*;
pub use self::model::*;
pub use self::move_piece::*;
pub use self::valid_movements_positions::*;

pub fn create_game(config: GameConfig) -> Game {
    Game::new(config)
}

/// Check if the given chess piece is a pawn and is on the starting position of the pawn color.
/// The starting position for white pawns is the second rank.
/// The starting position for black pawns is the seventh rank.
pub fn is_pawn_on_starting_position(piece: &ChessPiece) -> bool {
    let white_starting_row: ChessRank = 2.try_into().unwrap();
    let black_starting_row: ChessRank = 7.try_into().unwrap();

    let (row, _) = piece.position();

    if let PieceTypes::Pawn = piece.kind() {
        if let PieceColors::White = piece.color() {
            row == white_starting_row.to_index()
        } else {
            row == black_starting_row.to_index()
        }
    } else {
        false
    }
}

/// Get's all the possible movements, valid or invalid that a given piece can make.
/// The return value is a collections of paths the given piece can make. This makes easier for
/// checking for collisions down the line.
pub fn get_movement_pattern(piece: &ChessPiece) -> Vec<BoardPath> {
    let kind = *piece.kind();
    let (row, column) = piece.position();

    match kind {
        PieceTypes::Pawn => match (piece.color(), is_pawn_on_starting_position(piece)) {
            (PieceColors::Black, true) => {
                let positions = vec![(row - 1, column), (row - 2, column)];
                let path: Vec<BoardPosition> = positions
                    .into_iter()
                    .map(|s| s.try_into())
                    .filter(Result::is_ok)
                    .map(Result::unwrap)
                    .collect();
                vec![BoardPath(path)]
            }
            (PieceColors::Black, false) => {
                let position = (row - 1, column);
                if let Ok(position) = position.try_into() {
                    vec![BoardPath(vec![position])]
                } else {
                    vec![BoardPath(vec![])]
                }
            }
            (PieceColors::White, true) => {
                let positions = vec![(row + 1, column), (row + 2, column)];
                let path: Vec<BoardPosition> = positions
                    .into_iter()
                    .map(|s| s.try_into())
                    .filter(Result::is_ok)
                    .map(Result::unwrap)
                    .collect();
                vec![BoardPath(path)]
            }
            (PieceColors::White, false) => {
                let position = (row + 1, column);
                if let Ok(position) = position.try_into() {
                    vec![BoardPath(vec![position])]
                } else {
                    vec![BoardPath(vec![])]
                }
            }
        },
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

/// Creates a pattern of movement from a vec of positions.
/// Each position assumes that the piece is located at 0,0. Negative positions are allowed.
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
            0 => (1..(7 - row))
                .filter_map(|i| (row + i, column).try_into().ok())
                .collect(),
            1 => (1..(7 - column))
                .filter_map(|i| (row, column + i).try_into().ok())
                .collect(),
            2 => (1..row)
                .filter_map(|i| (row - i, column).try_into().ok())
                .collect(),
            3 => (1..column)
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
