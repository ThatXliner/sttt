//! This module contains the errors that [`super_ttt`](crate) may return.
use std::error::Error;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
/// Making a move wasn't possible. Used by [`super_ttt::Game::make_move`][crate::Game::make_move]
pub enum InvalidMoveError {
    /// The specified cell is already occupied
    CellAlreadyOccupied,
    /// The specified board does not match the coordinates of the opponent's last move
    InvalidBoard,
}

impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidMoveError::CellAlreadyOccupied => {
                "the specified cell is already occupied".fmt(f)
            }
            InvalidMoveError::InvalidBoard => {
                "the specified board does not match the coordinates of the opponent's last move"
                    .fmt(f)
            }
        }
    }
}

impl Error for InvalidMoveError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self {
            InvalidMoveError::CellAlreadyOccupied => "the specified cell is already occupied",
            InvalidMoveError::InvalidBoard => "you can't move in that board",
        }
    }
}
