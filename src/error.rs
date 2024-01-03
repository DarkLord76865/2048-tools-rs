//! A module that contains the Error enum.


use std::fmt::{self, Display, Formatter};
use std::error::Error as StdError;


/// An enum that represents the possible errors that can occur in this crate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// Invalid game size. Must be at least 4.
    InvalidSize,
    /// Invalid board provided. Board must be quadratic.
    InvalidBoard,
    /// Invalid value in a board. Must be 0 or power of 2, starting from 2.
    InvalidValue,
    /// There is no valid move to make. The game is over.
    NoValidMove,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::InvalidSize => write!(f, "Invalid game size. Must be at least 4."),
            Error::InvalidBoard => write!(f, "Invalid board provided. Board must be quadratic."),
            Error::InvalidValue => write!(f, "Invalid value in a board. Must be 0 or power of 2, starting from 2."),
            Error::NoValidMove => write!(f, "There is no valid move to make. The game is over."),
        }
    }
}
impl StdError for Error {}
