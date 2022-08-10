use nom::{
    branch::alt, bytes::streaming::tag,
    character::complete::alphanumeric1, combinator::not,
    error::VerboseError, sequence::terminated,
};

mod atom;
mod expression;

pub use atom::parse_atom;
pub use expression::parse_expression;

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;

/// Parses all words considered to be reserved.
///
/// Used as an auxiliary parser to guarantee that reserved words
/// aren't used as identifiers
fn parse_reserved_word(input: &str) -> IResult<&str> {
    terminated(
        alt((
            tag("if"),
            tag("true"),
            tag("false"),
            tag("nil"),
            tag("def"),
            tag("fn"),
        )),
        not(alphanumeric1),
    )(input)
}
