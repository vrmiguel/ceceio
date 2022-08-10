use crate::{
    parse_expression, Env, Error, Evaluable, Expression, Result,
};

#[derive(Default)]
pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::default(),
        }
    }

    pub fn parse_and_eval(
        &mut self,
        input: &str,
    ) -> Result<Expression> {
        self.eval(Self::parse(input)?)
    }

    pub fn parse(input: &str) -> Result<Expression> {
        let (rest, expr) = parse_expression(input)
            .map_err(Self::stringify_error)?;

        // assert!(rest.trim().is_empty());

        Ok(expr)
    }

    pub fn eval(
        &mut self,
        expression: Expression,
    ) -> Result<Expression> {
        expression.evaluate(&mut self.env)
    }

    fn stringify_error(
        error: nom::Err<nom::error::VerboseError<&str>>,
    ) -> Error {
        Error::ParsingError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Atom, Expression, Interpreter};

    #[test]
    fn parses_and_evaluates() {
        let mut interp = Interpreter::new();

        assert_eq!(
            interp
                .parse_and_eval("(/ (* 2 3) (- 5 6 7))")
                .unwrap(),
            Expression::Atom(Atom::Number(-0.75))
        );

        assert_eq!(
            interp.parse_and_eval("(def x 5)").unwrap(),
            Expression::Atom(Atom::Number(5.0))
        );

        assert_eq!(
            interp
                .parse_and_eval("(def twice (+ x x))")
                .unwrap(),
            Expression::Atom(Atom::Number(10.0))
        );

        assert_eq!(
            interp
                .parse_and_eval("(= twice (* x 2) 10.0)")
                .unwrap(),
            Expression::Atom(Atom::Boolean(true))
        );

        assert_eq!(
            interp
                .parse_and_eval("(= (* x x x x x) 3125.0)")
                .unwrap(),
            Expression::Atom(Atom::Boolean(true))
        );
    }
}
