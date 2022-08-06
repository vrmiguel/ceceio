use nom::error::VerboseError;

mod atom;
mod expression;

pub use atom::parse_atom;
pub use expression::parse_expression;

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;
