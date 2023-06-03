use super::{ArrayIndex, FromArrayIndexError};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ChessFile(char);
impl std::fmt::Display for ChessFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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

    fn from_index(value: usize) -> Result<Self, FromArrayIndexError> {
        match value {
            0..=7 => Ok(ChessFile(
                char::from_digit((97 + value) as u32, 10).unwrap(),
            )),
            8..=usize::MAX => Err(FromArrayIndexError::IndexTooBig(7)),
            _ => unreachable!(),
        }
    }
}
