#[derive(Debug)]
pub enum PieceColors {
    Black,
    White,
}

#[derive(Debug)]
pub enum PieceTypes {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug)]
pub struct BoardPosition {
    row: ChessRank,
    column: ChessFile,
}

#[derive(Debug)]
pub struct ChessPiece {
    kind: PieceTypes,
    position: BoardPosition,
    owner: PieceColors,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    color: PieceColors,
    ms_remaining: u64,
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<ChessCell>>,
}

#[derive(Debug)]
pub enum ChessCell {
    Empty,
    WhitePiece,
    BlackPiece,
}

#[derive(Debug)]
pub struct ChessRank(usize);
#[derive(Debug)]
pub struct ChessFile(char);

pub trait ArrayIndex {
    fn to_index(&self) -> usize;
}

pub enum FileInstancingErrors {
    InvalidColumn,
}
impl ChessFile {
    pub fn new(column: char) -> Result<Self, FileInstancingErrors> {
        match column {
            'a'..='h' => Ok(ChessFile(column)),
            _ => Err(FileInstancingErrors::InvalidColumn),
        }
    }
}

impl ArrayIndex for ChessFile {
    fn to_index(&self) -> usize {
        let base = 10;
        let digit = self
            .0
            .to_digit(base)
            .expect("Couldn't convert character to base 10.") as usize;
        digit - 97
    }
}

pub enum RankInstancingErrors {
    InvalidRow,
}
impl ChessRank {
    pub fn new(row: usize) -> Result<Self, RankInstancingErrors> {
        match row {
            1..=8 => Ok(ChessRank(row)),
            _ => Err(RankInstancingErrors::InvalidRow),
        }
    }
}

impl ArrayIndex for ChessRank {
    fn to_index(&self) -> usize {
        self.0 - 1
    }
}
