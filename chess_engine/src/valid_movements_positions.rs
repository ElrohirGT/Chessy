use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_get_valid_movements_positions(
    piece: JsValue,
    board: JsValue,
) -> Result<JsValue, JsValue> {
    let piece: ChessPiece = serde_wasm_bindgen::from_value(piece)?;
    let board: Board = serde_wasm_bindgen::from_value(board)?;
    let positions = get_valid_movements_positions(&piece, &board);

    Ok(serde_wasm_bindgen::to_value(&positions)?)
}

/// Get's all the valid positions this piece can move on the given board.
pub fn get_valid_movements_positions(piece: &ChessPiece, board: &Board) -> Vec<BoardPosition> {
    let mut pattern = get_movement_pattern(piece);

    if let PieceTypes::King = piece.kind() {
        return pattern
            .into_iter()
            .flat_map(|path| path.0)
            .filter(|pos| board.position_in_check(pos, piece.color()))
            .collect();
    }

    let pawn_capture_pattern: Option<Vec<BoardPath>> = board.get_capture_pattern(piece);
    if let (PieceTypes::Pawn, Some(mut paths)) = (piece.kind(), pawn_capture_pattern) {
        pattern.append(&mut paths);
    }

    let paths: Vec<BoardPath> = board.colission_detection(pattern, piece);
    let positions: Vec<BoardPosition> = paths.into_iter().flat_map(|path| path.0).collect();

    if let Some(check_positions) = board.get_check_positions() {
        positions
            .into_iter()
            .filter(|pos| check_positions.contains(pos))
            .collect()
    } else {
        positions
    }
}
