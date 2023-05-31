use thiserror::Error;

#[derive(Debug, Clone)]
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

#[derive(Debug, Error)]
pub enum BoardPositionInstancingErrors {
    #[error("The format of the file (column) is invalid.")]
    InvalidColumnFormat(FileInstancingErrors),
    #[error("The format of the rank (row) is invalid.")]
    InvalidRowFormat(RankInstancingErrors),
    #[error("The rank (`{0}`) must be a number.")]
    RankMustBeANumber(String),
    #[error("The value (`{0}`) must be of length 2.")]
    ValueTooLarge(String),
}

impl TryFrom<&str> for BoardPosition {
    type Error = BoardPositionInstancingErrors;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() == 2 {
            let column = ChessFile::try_from(value.chars().nth(0).unwrap())
                .map_err(|inner| BoardPositionInstancingErrors::InvalidColumnFormat(inner))?;
            let row: usize = value.get(1..=1).unwrap().parse().map_err(|_| {
                BoardPositionInstancingErrors::RankMustBeANumber(
                    value.get(1..=1).unwrap().to_string(),
                )
            })?;
            let row = ChessRank::try_from(row)
                .map_err(|inner| BoardPositionInstancingErrors::InvalidRowFormat(inner))?;
            Ok(BoardPosition { column, row })
        } else {
            Err(BoardPositionInstancingErrors::ValueTooLarge(
                value.to_string(),
            ))
        }
    }
}

#[derive(Debug)]
pub struct ChessPiece {
    kind: PieceTypes,
    position: BoardPosition,
    owner: PieceColors,
}

impl ChessPiece {
    pub fn new(kind: PieceTypes, position: BoardPosition, owner: PieceColors) -> ChessPiece {
        ChessPiece {
            kind,
            position,
            owner,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    color: PieceColors,
    ms_remaining: u64,
}

impl Player {
    pub fn new(name: String, color: PieceColors, ms_remaining: u64) -> Self {
        Player {
            name,
            color,
            ms_remaining,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<ChessCell>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<ChessCell>>) -> Board {
        Board { cells }
    }
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

#[derive(Debug, Error)]
pub enum FileInstancingErrors {
    #[error("The column must be a char between 'a' and 'h'!")]
    InvalidColumnChar,
}

impl TryFrom<char> for ChessFile {
    type Error = FileInstancingErrors;
    fn try_from(column: char) -> Result<Self, Self::Error> {
        match column {
            'a'..='h' => Ok(ChessFile(column)),
            _ => Err(FileInstancingErrors::InvalidColumnChar),
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

#[derive(Debug, Error)]
pub enum RankInstancingErrors {
    #[error("The row must be a number between 1 and 8!")]
    InvalidRowChar,
}

impl TryFrom<usize> for ChessRank {
    type Error = RankInstancingErrors;
    fn try_from(row: usize) -> Result<Self, Self::Error> {
        match row {
            1..=8 => Ok(ChessRank(row)),
            _ => Err(RankInstancingErrors::InvalidRowChar),
        }
    }
}

impl ArrayIndex for ChessRank {
    fn to_index(&self) -> usize {
        self.0 - 1
    }
}
