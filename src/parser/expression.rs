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
            Lambda,
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
            parse_binding.map(Box::new).map(Expression::Binding),
            parse_lambda.map(Box::new).map(Expression::Lambda),
            parse_application.map(Expression::Application),
            parse_cond.map(Expression::Cond),
            parse_list.map(Expression::List),
            parse_quote.map(Expression::List),
        )),
    )(input)
}

fn parse_list(input: &str) -> IResult<Vec<Expression>> {
    parse_square_brackets_enclosed(many0(parse_expression))(
        input,
    )
}

fn parse_quote(input: &str) -> IResult<Vec<Expression>> {
    preceded(
        multispace0,
        delimited(
            tag("'("),
            preceded(multispace0, many0(parse_expression)),
            context(
                "closing brackets",
                cut(preceded(multispace0, char(')'))),
            ),
        ),
    )(input)
}

fn parse_identifier_list(
    input: &str,
) -> IResult<Vec<SmallString>> {
    parse_square_brackets_enclosed(many0(preceded(
        multispace0,
        parse_identifier.map(SmallString::new),
    )))(input)
}

fn parse_square_brackets_enclosed<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<T>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    preceded(
        multispace0,
        delimited(
            char('['),
            preceded(multispace0, inner),
            context(
                "closing brackets",
                cut(preceded(multispace0, char(']'))),
            ),
        ),
    )
}

fn parse_lambda(input: &str) -> IResult<Lambda> {
    fn parse_lambda_inner(input: &str) -> IResult<Lambda> {
        let (rest, _) = tag("fn ")(input)?;
        let (rest, arguments) = parse_identifier_list(rest)?;
        let (rest, body) = parse_expression(rest)?;

        let lambda = Lambda { arguments, body };
        Ok((rest, lambda))
    }

    parse_parenthesis_enclosed(parse_lambda_inner)(input)
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

fn parse_cond(input: &str) -> IResult<Vec<Expression>> {
    fn parse_cond_inner(
        input: &str,
    ) -> IResult<Vec<Expression>> {
        let (rest, _) = tag("cond")(input)?;

        many0(parse_expression)(rest)
    }

    parse_parenthesis_enclosed(parse_cond_inner)(input)
}

#[cfg(test)]
mod tests {
    use super::{
        parse_application, parse_binding, parse_identifier_list,
        parse_lambda, Application,
    };
    use crate::{
        expression::{
            elements::{
                Atom, Binding, FnIdentifier, If, IfElse, Lambda,
            },
            BuiltIn, Expression,
        },
        parse_expression,
        parser::expression::{parse_cond, parse_if, parse_list},
        SmallString,
    };

    #[test]
    fn parses_applications() {
        assert_eq!(
            parse_application("(nothing)"),
            Ok((
                "",
                Application {
                    name: FnIdentifier::Other(SmallString::new(
                        "nothing"
                    )),
                    arguments: vec![]
                }
            ))
        );

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
    fn parses_lambdas() {
        assert_eq!(
            parse_lambda("(fn [] 2)"),
            Ok((
                "",
                Lambda {
                    arguments: vec![],
                    body: 2.0.into()
                }
            ))
        );

        assert_eq!(
            parse_lambda("(fn [x] (+ x x))"),
            Ok((
                "",
                Lambda {
                    arguments: vec![SmallString::new("x")],
                    body: Expression::Application(Application {
                        name: FnIdentifier::BuiltIn(
                            BuiltIn::Plus
                        ),
                        arguments: vec![
                            Expression::Atom(
                                Atom::Identifier(
                                    SmallString::new("x")
                                )
                            );
                            2
                        ]
                    })
                }
            ))
        );
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

    #[test]
    fn parses_argument_lists() {
        assert_eq!(
            parse_identifier_list("[] "),
            Ok((" ", vec![]))
        );
        assert_eq!(
            parse_identifier_list("[ ]"),
            Ok(("", vec![]))
        );
        assert_eq!(
            parse_identifier_list("[x y]"),
            Ok((
                "",
                vec![
                    SmallString::new("x"),
                    SmallString::new("y")
                ]
            ))
        );
        assert_eq!(
            parse_identifier_list(" [ x y z w e f     ] "),
            Ok((
                " ",
                vec![
                    SmallString::new("x"),
                    SmallString::new("y"),
                    SmallString::new("z"),
                    SmallString::new("w"),
                    SmallString::new("e"),
                    SmallString::new("f"),
                ]
            ))
        );
    }

    #[test]
    fn parses_cond_expressions() {
        assert_eq!(parse_cond("(cond)"), Ok(("", vec![])));

        assert_eq!(
            parse_cond("(cond true 2)"),
            Ok((
                "",
                vec![
                    Expression::Atom(Atom::Boolean(true)),
                    2.0.into()
                ]
            ))
        );

        assert_eq!(
            parse_cond("(cond false 2 true 5)"),
            Ok((
                "",
                vec![
                    Expression::Atom(Atom::Boolean(false)),
                    2.0.into(),
                    Expression::Atom(Atom::Boolean(true)),
                    5.0.into()
                ]
            ))
        );

        assert!(parse_cond("(cond false 2 true)").is_ok());
    }

    #[test]
    fn parses_lists() {
        assert_eq!(
            parse_list("[true false]"),
            Ok((
                "",
                vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::Atom(Atom::Boolean(false)),
                ]
            ))
        );

        assert_eq!(
            parse_expression("[true false]"),
            Ok((
                "",
                Expression::List(vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::Atom(Atom::Boolean(false)),
                ])
            ))
        );

        assert_eq!(
            parse_expression("[true [true false]]"),
            Ok((
                "",
                Expression::List(vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::List(vec![
                        Expression::Atom(Atom::Boolean(true)),
                        Expression::Atom(Atom::Boolean(false)),
                    ]),
                ])
            ))
        );

        assert_eq!(
            parse_expression("[true [true [true false]]]"),
            Ok((
                "",
                Expression::List(vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::List(vec![
                        Expression::Atom(Atom::Boolean(true)),
                        Expression::List(vec![
                            Expression::Atom(Atom::Boolean(
                                true
                            )),
                            Expression::Atom(Atom::Boolean(
                                false
                            )),
                        ]),
                    ]),
                ])
            ))
        );
    }

    #[test]
    fn parses_quotes() {
        assert_eq!(
            parse_expression("'(true false)"),
            Ok((
                "",
                Expression::List(vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::Atom(Atom::Boolean(false)),
                ])
            ))
        );

        assert_eq!(
            parse_expression("'(true [true false])"),
            Ok((
                "",
                Expression::List(vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::List(vec![
                        Expression::Atom(Atom::Boolean(true)),
                        Expression::Atom(Atom::Boolean(false)),
                    ]),
                ])
            ))
        );

        assert_eq!(
            parse_expression("'(true [true '(true false)])"),
            Ok((
                "",
                Expression::List(vec![
                    Expression::Atom(Atom::Boolean(true)),
                    Expression::List(vec![
                        Expression::Atom(Atom::Boolean(true)),
                        Expression::List(vec![
                            Expression::Atom(Atom::Boolean(
                                true
                            )),
                            Expression::Atom(Atom::Boolean(
                                false
                            )),
                        ]),
                    ]),
                ])
            ))
        );
    }
}
