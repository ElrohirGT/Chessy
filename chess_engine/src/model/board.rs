use super::{BoardPosition, ChessCell, PieceColors};

#[derive(Debug)]
pub struct Board {
    pub color_in_check: Option<PieceColors>,
    pub cells: Vec<Vec<ChessCell>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<ChessCell>>, color_in_check: Option<PieceColors>) -> Board {
        Board {
            cells,
            color_in_check,
        }
    }

    pub fn color_in_check(&self, owner: &PieceColors) -> bool {
        if let Some(color) = &self.color_in_check {
            color == owner
        } else {
            false
        }
    }

    pub fn get_cell(&self, position: BoardPosition) -> &ChessCell {
        let (row, column): (usize, usize) = position.into();
        (&self).cells.get(row).unwrap().get(column).unwrap()
    }
}
