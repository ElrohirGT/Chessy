use super::{ArrayIndex, FromArrayIndexError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        let digit = self.0 as u8;
        (digit as usize) - 97usize
    }

    fn from_index(value: usize) -> Result<Self, FromArrayIndexError> {
        match value {
            0..=7 => Ok(ChessFile(((value as u8) + 97) as char)),
            8..=usize::MAX => Err(FromArrayIndexError::IndexTooBig(7)),
            _ => unreachable!(),
        }
    }
}
