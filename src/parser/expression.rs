use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{cut, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    Parser,
};

use crate::{
    expression::{
        elements::{
            Application, Binding, FnIdentifier, If, IfElse,
        },
        Expression,
    },
    parse_atom,
    parser::atom::{parse_fn_identifier, parse_identifier},
    IResult, SmallString,
};

pub fn parse_expression(input: &str) -> IResult<Expression> {
    preceded(
        multispace0,
        alt((
            parse_atom.map(Expression::Atom),
            parse_if,
            parse_application.map(Expression::Application),
        )),
    )(input)
}

fn parse_if(input: &str) -> IResult<Expression> {
    fn parse_if_inner(input: &str) -> IResult<Expression> {
        let (rest, (condition, if_true, if_false)) =
            preceded(
                terminated(tag("if"), multispace1),
                cut(tuple((
                    parse_expression,
                    parse_expression,
                    opt(parse_expression),
                ))),
            )(input)?;

        let expr = match if_false {
            Some(if_false) => {
                Expression::IfElse(Box::new(IfElse {
                    condition,
                    if_true,
                    if_false,
                }))
            }
            None => Expression::If(Box::new(If {
                condition,
                do_this: if_true,
            })),
        };

        Ok((rest, expr))
    }

    parse_parenthesis_enclosed(parse_if_inner)(input)
}

fn parse_binding(input: &str) -> IResult<Binding> {
    fn parse_identifier_and_expr(
        input: &str,
    ) -> IResult<(&str, Expression)> {
        let (rest, _) = tag("def ")(input)?;
        let (rest, identifier) = parse_identifier(rest)?;
        let (rest, expression) = parse_expression(rest)?;

        Ok((rest, (identifier, expression)))
    }
    let (rest, (identifier, expression)) =
        parse_parenthesis_enclosed(parse_identifier_and_expr)(
            input,
        )?;
    let binding = Binding {
        identifier: SmallString::new(identifier),
        expression,
    };
    Ok((rest, binding))
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
    use super::{parse_application, parse_binding, Application};
    use crate::{
        expression::{
            elements::{
                Atom, Binding, FnIdentifier, If, IfElse,
            },
            BuiltIn, Expression,
        },
        parse_expression,
        parser::expression::parse_if,
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

        assert_eq!(
            parse_application("(+ (- 2 3) 5)"),
            Ok((
                "",
                Application {
                    name: FnIdentifier::BuiltIn(BuiltIn::Plus),
                    arguments: vec![
                        Expression::Application(Application {
                            name: FnIdentifier::BuiltIn(
                                BuiltIn::Minus
                            ),
                            arguments: vec![
                                Expression::Atom(Atom::Number(
                                    2.0
                                )),
                                Expression::Atom(Atom::Number(
                                    3.0
                                ))
                            ]
                        },),
                        Expression::Atom(Atom::Number(5.0))
                    ]
                }
            ))
        );
    }

    #[test]
    fn parses_if_statements() {
        assert_eq!(
            parse_if("(if true 2)"),
            Ok((
                "",
                Expression::If(Box::new(If {
                    condition: Expression::Atom(Atom::Boolean(
                        true
                    )),
                    do_this: Expression::Atom(Atom::Number(2.0)),
                }))
            ))
        );

        assert_eq!(
            parse_if("(if true 2 3)"),
            Ok((
                "",
                Expression::IfElse(Box::new(IfElse {
                    condition: Expression::Atom(Atom::Boolean(
                        true
                    )),
                    if_true: Expression::Atom(Atom::Number(2.0)),
                    if_false: Expression::Atom(Atom::Number(
                        3.0
                    ))
                }))
            ))
        );

        assert!(parse_if(
            "(if (= (div 3 2) 0) 2 (= (div 5 2) 0))"
        )
        .is_ok());
    }

    #[test]
    fn parses_bindings() {
        assert_eq!(
            parse_binding("(def two 2)"),
            Ok((
                "",
                Binding {
                    identifier: SmallString::new("two"),
                    expression: Expression::Atom(Atom::Number(
                        2.0
                    ))
                }
            ))
        );

        assert_eq!(
            parse_binding("(def five (+ 2 3))"),
            Ok((
                "",
                Binding {
                    identifier: SmallString::new("five"),
                    expression: Expression::Application(
                        Application {
                            name: FnIdentifier::BuiltIn(
                                BuiltIn::Plus
                            ),
                            arguments: vec![
                                Expression::Atom(Atom::Number(
                                    2.0
                                )),
                                Expression::Atom(Atom::Number(
                                    3.0
                                ))
                            ]
                        }
                    )
                }
            ))
        );

        assert!(parse_binding("(def)").is_err());
        assert!(parse_binding("(def x)").is_err());
    }

    #[test]
    fn parses_expressions() {
        assert_eq!(
            parse_expression("5"),
            Ok(("", Expression::Atom(Atom::Number(5.0))))
        );

        assert_eq!(
            parse_expression("(add 2 3)"),
            Ok((
                "",
                Expression::Application(Application {
                    name: FnIdentifier::Other(SmallString::new(
                        "add"
                    )),
                    arguments: vec![
                        Expression::Atom(Atom::Number(2.0)),
                        Expression::Atom(Atom::Number(3.0))
                    ]
                })
            ))
        );

        assert_eq!(
            parse_expression("(if false 5 4)"),
            Ok((
                "",
                Expression::IfElse(Box::new(IfElse {
                    condition: Expression::Atom(Atom::Boolean(
                        false
                    )),
                    if_true: Expression::Atom(Atom::Number(5.0)),
                    if_false: Expression::Atom(Atom::Number(
                        4.0
                    ))
                }))
            ))
        );

        assert!(parse_expression(
            "(add (if true 2 3) (- (* 3 4 (/ 3 4)) 5))"
        )
        .is_ok());
    }
}
