use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{
        alphanumeric1, digit1, none_of, one_of,
    },
    combinator::{cut, not, recognize, value},
    error::context,
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated},
    Parser,
};

use super::{parse_reserved_word, IResult};
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
            value(Atom::Nil, tag("nil")),
            parse_symbol.map(SmallString::new).map(Atom::Symbol),
            parse_string.map(SmallString::new).map(Atom::String),
            parse_identifier
                .map(SmallString::new)
                .map(Atom::Identifier),
        )),
    )(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str> {
    let acceptable_chars = |ch: char| {
        ch.is_ascii_alphanumeric()
            || matches!(ch, '-' | '_' | '?')
    };

    let (rest, identifier) = recognize(pair(
        // Ensure that the identifier doesn't start with a
        // digit
        not(digit1),
        take_while1(acceptable_chars),
    ))(input)?;

    not(parse_reserved_word)(input)?;

    Ok((rest, identifier))
}

fn parse_symbol(input: &str) -> IResult<&str> {
    context(
        "symbol",
        preceded(tag(":"), cut(parse_identifier)),
    )(input)
}

#[inline(always)]
fn parse_builtin(input: &str) -> IResult<BuiltIn> {
    context(
        "builtin",
        alt((
            parse_operator,
            value(
                BuiltIn::Not,
                terminated(tag("not"), not(alphanumeric1)),
            ),
            value(
                BuiltIn::And,
                terminated(tag("and"), not(alphanumeric1)),
            ),
            value(
                BuiltIn::Or,
                terminated(tag("or"), not(alphanumeric1)),
            ),
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
            parse_identifier
                .map(SmallString::new)
                .map(FnIdentifier::Other),
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
        context("operator", one_of("+-*/=%"))(input)?;

    let op = match op {
        '+' => BuiltIn::Plus,
        '-' => BuiltIn::Minus,
        '*' => BuiltIn::Times,
        '/' => BuiltIn::Divide,
        '=' => BuiltIn::Equal,
        '%' => BuiltIn::Remainder,
        _ => {
            unreachable!(
                "we checked that `op in [+-*/=%]` above"
            )
        }
    };

    Ok((rest, op))
}

#[inline(always)]
fn parse_double(input: &str) -> IResult<f64> {
    double(input)
}

fn parse_string(input: &str) -> IResult<&str> {
    let esc = escaped(none_of("\\\""), '\\', tag("\""));
    let esc_or_empty = alt((esc, tag("")));

    delimited(tag("\""), esc_or_empty, tag("\""))(input)
}

#[cfg(test)]
mod tests {
    use super::{parse_double, parse_string};
    use crate::{
        expression::{elements::Atom, BuiltIn},
        parser::atom::{
            parse_atom, parse_boolean, parse_builtin,
            parse_identifier, parse_operator, parse_symbol,
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
        assert_eq!(parse_symbol(":arg"), Ok(("", "arg")));
        assert_eq!(
            parse_symbol(":arg other"),
            Ok((" other", "arg"))
        );

        assert!(parse_symbol("arg1").is_err());
        assert!(parse_symbol(":1arg").is_err());
    }

    #[test]
    fn parses_builtins() {
        assert_eq!(
            parse_builtin("+ -/="),
            Ok((" -/=", BuiltIn::Plus))
        );
        assert_eq!(
            parse_builtin("- /=+"),
            Ok((" /=+", BuiltIn::Minus))
        );
        assert_eq!(
            parse_builtin("/ =+-"),
            Ok((" =+-", BuiltIn::Divide))
        );
        assert_eq!(
            parse_builtin("= +-/"),
            Ok((" +-/", BuiltIn::Equal))
        );
        assert_eq!(
            parse_builtin("not =+-/"),
            Ok((" =+-/", BuiltIn::Not))
        );
        assert_eq!(
            parse_builtin("and =+-/not"),
            Ok((" =+-/not", BuiltIn::And))
        );

        assert_eq!(
            parse_builtin("or =+and-/not"),
            Ok((" =+and-/not", BuiltIn::Or))
        );

        assert!(parse_builtin("a 1.2").is_err());
    }

    #[test]
    fn parses_identifiers() {
        assert_eq!(parse_identifier("arg"), Ok(("", "arg")));
        assert_eq!(
            parse_identifier("identifier other_identifier"),
            Ok((" other_identifier", "identifier"))
        );
        assert_eq!(
            parse_identifier("numeric123 123"),
            Ok((" 123", "numeric123"))
        );

        assert!(parse_identifier(":arg1").is_err());
        assert!(parse_identifier("1arg").is_err());

        // `def` is a reserved word so this must fail
        assert!(parse_identifier("def").is_err());
        assert!(parse_identifier("adef").is_ok());

        // `fn` is a reserved word so this must fail
        assert!(parse_identifier("fn").is_err());
        assert!(parse_identifier("fnn").is_ok());

        assert_eq!(
            parse_identifier("even? 123"),
            Ok((" 123", "even?"))
        );

        assert_eq!(
            parse_identifier("is-even? 123"),
            Ok((" 123", "is-even?"))
        );
    }

    #[test]
    fn parses_strings() {
        assert_eq!(parse_string("\"hey\""), Ok(("", "hey")));

        assert_eq!(
            parse_string("\"hello world\" hello"),
            Ok((" hello", "hello world"))
        );

        assert_eq!(
            parse_string("\"hello \\\"world\" hello"),
            Ok((" hello", "hello \\\"world"))
        );

        assert!(parse_string("\"missing closing quote").is_err());
        assert!(parse_string("missing opening quote\"").is_err());
    }

    #[test]
    fn parses_operators() {
        assert_eq!(
            parse_operator("+ -/="),
            Ok((" -/=", BuiltIn::Plus))
        );
        assert_eq!(
            parse_operator("- /=+"),
            Ok((" /=+", BuiltIn::Minus))
        );
        assert_eq!(
            parse_operator("/ =+-"),
            Ok((" =+-", BuiltIn::Divide))
        );
        assert_eq!(
            parse_operator("=+-/"),
            Ok(("+-/", BuiltIn::Equal))
        );
        assert_eq!(
            parse_operator("%=+-/"),
            Ok(("=+-/", BuiltIn::Remainder))
        );

        assert!(parse_double("a 1.2").is_err());
    }
}
