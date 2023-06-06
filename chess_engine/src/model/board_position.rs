use crate::{ArrayIndex, FileInstancingErrors, RankInstancingErrors};

pub use super::{ChessFile, ChessRank, FromMatrixPositionError, MatrixPosition};
use thiserror::Error;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
pub enum BoardPositionFromStrErrors {
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
    type Error = BoardPositionFromStrErrors;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() == 2 {
            let column = ChessFile::try_from(value.chars().next().unwrap())
                .map_err(BoardPositionFromStrErrors::InvalidColumnFormat)?;
            let row: usize = value.get(1..=1).unwrap().parse().map_err(|_| {
                BoardPositionFromStrErrors::RankMustBeANumber(value.get(1..=1).unwrap().to_string())
            })?;
            let row =
                ChessRank::try_from(row).map_err(BoardPositionFromStrErrors::InvalidRowFormat)?;
            Ok(BoardPosition { column, row })
        } else {
            Err(BoardPositionFromStrErrors::ValueTooLarge(value.to_string()))
        }
    }
}

#[derive(Debug, Error)]
pub enum BoardPositionFromIsizeErrors {
    #[error("The file (column) passed in is negative.")]
    NegativeFile,
    #[error("The rank (row) passed in is negative.")]
    NegativeRank,
    #[error("The rank (row) passed in is greater than 7.")]
    RankTooHigh,
    #[error("The file (column) passed in is greater than 7.")]
    FileTooHigh,
}

impl TryFrom<(isize, isize)> for BoardPosition {
    type Error = BoardPositionFromIsizeErrors;
    fn try_from((row, column): (isize, isize)) -> Result<Self, Self::Error> {
        if row < 0 {
            Err(BoardPositionFromIsizeErrors::NegativeRank)
        } else if column < 0 {
            Err(BoardPositionFromIsizeErrors::NegativeFile)
        } else {
            let row = row as usize;
            let column = column as usize;

            (row, column).try_into().map_err(|e| match e {
                BoardPositionFromUsizeErrors::FileTooHigh => {
                    BoardPositionFromIsizeErrors::FileTooHigh
                }
                BoardPositionFromUsizeErrors::RankTooHigh => {
                    BoardPositionFromIsizeErrors::RankTooHigh
                }
            })
        }
    }
}

#[derive(Debug, Error)]
pub enum BoardPositionFromUsizeErrors {
    #[error("The rank (row) passed in is greater than 7.")]
    RankTooHigh,
    #[error("The file (column) passed in is greater than 7.")]
    FileTooHigh,
}

impl TryFrom<(usize, usize)> for BoardPosition {
    type Error = BoardPositionFromUsizeErrors;
    fn try_from((row, column): (usize, usize)) -> Result<Self, Self::Error> {
        let row =
            ChessRank::from_index(row).map_err(|_| BoardPositionFromUsizeErrors::RankTooHigh)?;
        let column =
            ChessFile::from_index(column).map_err(|_| BoardPositionFromUsizeErrors::FileTooHigh)?;
        Ok(BoardPosition { row, column })
    }
}

impl Into<(usize, usize)> for BoardPosition {
    fn into(self) -> (usize, usize) {
        (self.row.to_index(), self.column.to_index())
    }
}

impl Into<(usize, usize)> for &BoardPosition {
    fn into(self) -> (usize, usize) {
        (self.row.to_index(), self.column.to_index())
    }
}
