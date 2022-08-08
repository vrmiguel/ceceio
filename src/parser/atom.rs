use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, one_of},
    combinator::{cut, not, recognize, value},
    error::context,
    number::complete::double,
    sequence::{pair, preceded},
    Parser,
};

use super::IResult;
use crate::{
    expression::{
        elements::{Atom, FnIdentifier},
        BuiltIn,
    },
    SmallString,
};

pub fn parse_atom(input: &str) -> IResult<Atom> {
    context(
        "atom",
        alt((
            parse_double.map(Atom::Number),
            parse_boolean.map(Atom::Boolean),
            parse_builtin.map(Atom::BuiltIn),
            parse_symbol.map(Atom::Symbol),
            parse_identifier.map(Atom::Identifier),
        )),
    )(input)
}

fn parse_symbol(input: &str) -> IResult<SmallString> {
    context(
        "symbol",
        preceded(tag(":"), cut(parse_identifier)),
    )(input)
}

fn parse_identifier(input: &str) -> IResult<SmallString> {
    let (rest, identifier) = recognize(pair(
        // Ensure that the identifier doesn't start with a
        // digit
        not(digit1),
        alphanumeric1,
    ))(input)?;

    Ok((rest, SmallString::new(identifier)))
}

#[inline(always)]
fn parse_builtin(input: &str) -> IResult<BuiltIn> {
    context(
        "builtin",
        alt((
            parse_operator,
            value(BuiltIn::Not, tag("not")),
            value(BuiltIn::And, tag("and")),
            value(BuiltIn::Or, tag("or")),
        )),
    )(input)
}

pub fn parse_fn_identifier(
    input: &str,
) -> IResult<FnIdentifier> {
    context(
        "identifier",
        alt((
            parse_builtin.map(FnIdentifier::BuiltIn),
            parse_identifier.map(FnIdentifier::Other),
        )),
    )(input)
}

#[inline(always)]
fn parse_boolean(input: &str) -> IResult<bool> {
    let (rest, boolean) =
        alt((tag("true"), tag("false")))(input)?;

    let is_true = boolean == "true";

    Ok((rest, is_true))
}

#[inline(always)]
fn parse_operator(input: &str) -> IResult<BuiltIn> {
    let (rest, op) =
        context("operator", one_of("+-*/="))(input)?;

    let op = match op {
        '+' => BuiltIn::Plus,
        '-' => BuiltIn::Minus,
        '*' => BuiltIn::Times,
        '/' => BuiltIn::Divide,
        '=' => BuiltIn::Equal,
        _ => {
            unreachable!("we checked that `op in [+-*/=]` above")
        }
    };

    Ok((rest, op))
}

#[inline(always)]
fn parse_double(input: &str) -> IResult<f64> {
    double(input)
}

#[cfg(test)]
mod tests {
    use super::parse_double;
    use crate::{
        expression::{elements::Atom, BuiltIn},
        parser::atom::{
            parse_atom, parse_boolean, parse_builtin,
            parse_operator, parse_symbol,
        },
        SmallString,
    };

    #[test]
    fn parses_atoms() {
        assert_eq!(
            parse_atom("2.3"),
            Ok(("", Atom::Number(2.3)))
        );

        assert_eq!(
            parse_atom("true"),
            Ok(("", Atom::Boolean(true)))
        );

        assert_eq!(
            parse_atom(":arg"),
            Ok(("", Atom::Symbol(SmallString::new("arg"))))
        );

        assert_eq!(
            parse_atom("+"),
            Ok(("", Atom::BuiltIn(BuiltIn::Plus)))
        );
    }

    #[test]
    fn parses_doubles() {
        assert_eq!(parse_double("2.3"), Ok(("", 2.3)));
        assert_eq!(parse_double("5.00 "), Ok((" ", 5.0)));
        assert_eq!(parse_double("1 1.2"), Ok((" 1.2", 1.0)));

        assert!(parse_double("a 1.2").is_err());
    }

    #[test]
    fn parses_booleans() {
        assert_eq!(parse_boolean("true"), Ok(("", true)));
        assert_eq!(parse_boolean("false"), Ok(("", false)));
        assert_eq!(
            parse_boolean("false false"),
            Ok((" false", false))
        );

        assert!(parse_boolean("False").is_err());
        assert!(parse_boolean("True").is_err());
        assert!(parse_boolean("1").is_err());
    }

    #[test]
    fn parses_symbols() {
        assert_eq!(
            parse_symbol(":arg"),
            Ok(("", SmallString::new("arg")))
        );
        assert_eq!(
            parse_symbol(":arg other"),
            Ok((" other", SmallString::new("arg")))
        );

        assert!(parse_symbol("arg1").is_err());
    }

    #[test]
    fn parses_builtins() {
        assert_eq!(
            parse_builtin("+-/="),
            Ok(("-/=", BuiltIn::Plus))
        );
        assert_eq!(
            parse_builtin("-/=+"),
            Ok(("/=+", BuiltIn::Minus))
        );
        assert_eq!(
            parse_builtin("/=+-"),
            Ok(("=+-", BuiltIn::Divide))
        );
        assert_eq!(
            parse_builtin("=+-/"),
            Ok(("+-/", BuiltIn::Equal))
        );
        assert_eq!(
            parse_builtin("not=+-/"),
            Ok(("=+-/", BuiltIn::Not))
        );
        assert_eq!(
            parse_builtin("and=+-/not"),
            Ok(("=+-/not", BuiltIn::And))
        );

        assert_eq!(
            parse_builtin("or=+and-/not"),
            Ok(("=+and-/not", BuiltIn::Or))
        );

        assert!(parse_double("a 1.2").is_err());
    }

    #[test]
    fn parses_operations() {
        assert_eq!(
            parse_operator("+-/="),
            Ok(("-/=", BuiltIn::Plus))
        );
        assert_eq!(
            parse_operator("-/=+"),
            Ok(("/=+", BuiltIn::Minus))
        );
        assert_eq!(
            parse_operator("/=+-"),
            Ok(("=+-", BuiltIn::Divide))
        );
        assert_eq!(
            parse_operator("=+-/"),
            Ok(("+-/", BuiltIn::Equal))
        );

        assert!(parse_double("a 1.2").is_err());
    }
}
