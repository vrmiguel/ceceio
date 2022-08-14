use crate::SmallString;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum Error {
    #[error(
        "Type error: expected {expected}, received {received}"
    )]
    TypeMismatch {
        expected: &'static str,
        received: &'static str,
    },
    #[error(
        "Arity mismatch: expected {expected}, received {received}"
    )]
    ExactArityMismatch { expected: u8, received: u8 },
    #[error(
        "Arity mismatch: expected at least {at_least}, received {received}"
    )]
    MinimumArityMismatch { at_least: u8, received: u8 },
    #[error("Unknown symbol {0}")]
    UnknownSymbol(SmallString),
    #[error("Parsing error: {0}")]
    ParsingError(String),
}
