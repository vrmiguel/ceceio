use nom::{
    branch::alt,
    bytes::complete::take_till,
    character::{
        complete::{char, multispace0},
        streaming::alphanumeric1,
    },
    combinator::cut,
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    Parser,
};

use crate::{
    expression::{Application, Expression, FnIdentifier},
    parse_atom,
    parser::atom::parse_fn_identifier,
    IResult, SmallString,
};

pub fn parse_expression(input: &str) -> IResult<Expression> {
    alt((parse_atom.map(Expression::Atom),))(input)
}

// Based on https://github.com/Geal/nom/blob/761ab0a24fccb4c560367b583b608fbae5f31647/examples/s_expression.rs#L155
fn parse_parenthesis_enclosed<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<T>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        context(
            "closing parenthesis",
            cut(preceded(multispace0, char(')'))),
        ),
    )
}

fn parse_application(input: &str) -> IResult<Application> {
    #[inline]
    fn parse_name_and_args(
        input: &str,
    ) -> IResult<(FnIdentifier, Vec<Expression>)> {
        let (rest, name) = parse_fn_identifier(input)?;

        let (rest, args) = many0(preceded(
            multispace0,
            parse_expression,
        ))(rest)?;

        Ok((rest, (name, args)))
    }

    let (rest, (name, arguments)) =
        parse_parenthesis_enclosed(parse_name_and_args)(input)?;

    Ok((rest, Application { name, arguments }))
}

#[cfg(test)]
mod tests {
    use super::{parse_application, Application};
    use crate::{
        expression::{Atom, BuiltIn, Expression, FnIdentifier},
        SmallString,
    };

    #[test]
    fn parses_applications() {
        assert_eq!(
            parse_application("(exit)"),
            Ok((
                "",
                Application {
                    name: FnIdentifier::Other(SmallString::new(
                        "exit"
                    )),
                    arguments: vec![]
                }
            ))
        );

        assert_eq!(
            parse_application("(+ 5 2 3)"),
            Ok((
                "",
                Application {
                    name: FnIdentifier::BuiltIn(BuiltIn::Plus),
                    arguments: vec![
                        Expression::Atom(Atom::Number(5.0)),
                        Expression::Atom(Atom::Number(2.0)),
                        Expression::Atom(Atom::Number(3.0))
                    ]
                }
            ))
        );
    }
}
