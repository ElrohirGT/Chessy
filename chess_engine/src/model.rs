use enum_iterator::Sequence;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PieceColors {
    Black,
    White,
}

impl std::fmt::Display for PieceColors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            PieceColors::Black => write!(f, "Black"),
            PieceColors::White => write!(f, "White"),
        }
    }
}

#[derive(Debug, Sequence, Clone, PartialEq, Eq)]
pub enum PieceTypes {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct BoardPosition {
    pub row: ChessRank,
    pub column: ChessFile,
}

impl std::fmt::Display for BoardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.column, self.row)
    }
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
            let column = ChessFile::try_from(value.chars().next().unwrap())
                .map_err(BoardPositionInstancingErrors::InvalidColumnFormat)?;
            let row: usize = value.get(1..=1).unwrap().parse().map_err(|_| {
                BoardPositionInstancingErrors::RankMustBeANumber(
                    value.get(1..=1).unwrap().to_string(),
                )
            })?;
            let row = ChessRank::try_from(row)
                .map_err(BoardPositionInstancingErrors::InvalidRowFormat)?;
            Ok(BoardPosition { column, row })
        } else {
            Err(BoardPositionInstancingErrors::ValueTooLarge(
                value.to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChessPiece {
    kind: PieceTypes,
    position: BoardPosition,
    pub owner: PieceColors,
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
}

#[derive(Debug)]
pub struct ChessCell(pub Option<ChessPiece>);
impl ChessCell {
    pub fn some(piece: ChessPiece) -> ChessCell {
        ChessCell(Some(piece))
    }
    pub fn none() -> ChessCell {
        ChessCell(None)
    }
    pub fn is_occupied(&self) -> bool {
        self.0.is_some()
    }
    pub fn piece_has_color(&self, color: &PieceColors) -> bool {
        if let Some(piece) = &self.0 {
            &piece.owner == color
        } else {
            false
        }
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct ChessRank(usize);
impl std::fmt::Display for ChessRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Clone)]
pub struct ChessFile(char);
impl std::fmt::Display for ChessFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
