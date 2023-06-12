use super::{ArrayIndex, FromArrayIndexError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessRank(usize);
impl std::fmt::Display for ChessRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

#[derive(Debug, Error)]
pub enum RankInstancingErrors {
    #[error("The rank (row) must be a number between 1 and 8!")]
    InvalidRowNumber,
}

impl TryFrom<usize> for ChessRank {
    type Error = RankInstancingErrors;
    fn try_from(row: usize) -> Result<Self, Self::Error> {
        match row {
            1..=8 => Ok(ChessRank(row)),
            _ => Err(RankInstancingErrors::InvalidRowNumber),
        }
    }
}

impl ArrayIndex for ChessRank {
    fn to_index(&self) -> usize {
        self.0 - 1
    }

    fn from_index(value: usize) -> Result<Self, FromArrayIndexError> {
        match value {
            0..=7 => Ok(ChessRank(value + 1)),
            8..=usize::MAX => Err(FromArrayIndexError::IndexTooBig(7)),
            _ => unreachable!(),
        }
    }
}
