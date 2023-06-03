mod game_model;
mod model;

use thiserror::Error;

pub use self::game_model::*;
pub use self::model::*;

pub fn create_game(config: GameConfig) -> Game {
    Game::new(config)
}

pub fn get_valid_movements_positions(piece: &ChessPiece, board: &Board) -> Vec<BoardPosition> {
    let pattern = get_movement_pattern(piece, board);
    todo!()
}
/// Get's all the possible movements, valid or invalid that a given piece can make.
/// The return value is a collections of paths the given piece can make. This makes easier for
/// checking for collisions down the line.
pub fn get_movement_pattern(piece: &ChessPiece, board: &Board) -> Vec<BoardPath> {
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
                .filter_map(|(i)| {
                    (row as isize + i * r, column as isize + i * c)
                        .try_into()
                        .ok()
                })
                .collect::<Vec<BoardPosition>>()
        })
        .map(BoardPath::from)
        .collect()
}
