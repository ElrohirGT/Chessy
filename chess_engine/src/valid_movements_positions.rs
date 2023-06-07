use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_get_valid_movements_positions(
    piece: JsValue,
    board: JsValue,
) -> Result<JsValue, JsValue> {
    let piece: ChessPiece = serde_wasm_bindgen::from_value(piece)?;
    let board: Board = serde_wasm_bindgen::from_value(board)?;
    let positions = get_valid_movements_positions(piece, board);

    Ok(serde_wasm_bindgen::to_value(&positions)?)
}

/// Get's all the valid positions this piece can move on the given board.
pub fn get_valid_movements_positions(piece: ChessPiece, board: Board) -> Vec<BoardPosition> {
    let mut pattern = get_movement_pattern(&piece);

    if let PieceTypes::King = piece.kind() {
        return pattern
            .into_iter()
            .map(|path| path.0)
            .flatten()
            .filter(|pos| position_in_check(pos, piece.color(), &board))
            .collect();
    }

    let pawn_capture_pattern: Option<Vec<BoardPath>> = get_capture_pattern(&piece, &board);
    if let (PieceTypes::Pawn, Some(mut paths)) = (piece.kind(), pawn_capture_pattern) {
        pattern.append(&mut paths);
    }

    let paths: Vec<BoardPath> = colission_detection(pattern, &piece, &board);
    let positions: Vec<BoardPosition> = paths.into_iter().map(|path| path.0).flatten().collect();

    if let Some(check_positions) = board.get_check_positions() {
        positions
            .into_iter()
            .filter(|pos| check_positions.contains(&pos))
            .collect()
    } else {
        positions
    }
}

/// Checks if a king of the given color in the given position would be in check in the given board.
/// Returns true if the positions would be in check.
pub fn position_in_check(position: &BoardPosition, color: &PieceColors, board: &Board) -> bool {
    let opponent_pieces = board.get_pieces_from(color.opponent());
    opponent_pieces
        .iter()
        .map(|piece| get_movement_paths(piece, board))
        .flatten()
        .map(|path: BoardPath| path.0)
        .flatten()
        .any(|pos| pos == *position)
}

/// Get's all the valid movement paths the piece can take, considering collisions with other pieces.
pub(crate) fn get_movement_paths(piece: &ChessPiece, board: &Board) -> Vec<BoardPath> {
    let mut pattern = get_movement_pattern(piece);
    let pawn_capture_pattern: Option<Vec<BoardPath>> = get_capture_pattern(piece, board);

    if let (PieceTypes::Pawn, Some(mut paths)) = (piece.kind(), pawn_capture_pattern) {
        pattern.append(&mut paths);
    }

    let paths: Vec<BoardPath> = colission_detection(pattern, piece, board);
    paths
}

/// Checks all the paths and checks for collisions with another pieces.
/// Returns the new paths now taking into account collision with other pieces.
fn colission_detection(
    pattern: Vec<BoardPath>,
    piece: &ChessPiece,
    board: &Board,
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

                    let piece_option = board.get_piece(&position);
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

/// Get's the movement pattern of this piece if it can capture a piece.
/// The only piece that actually needs this method is the pawn, because it can capture diagonally.
fn get_capture_pattern(piece: &ChessPiece, board: &Board) -> Option<Vec<BoardPath>> {
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
                board.get_piece(&((row as isize + r, column as isize + c).try_into().ok()?))
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
