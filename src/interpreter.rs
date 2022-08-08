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

        assert!(rest.trim().is_empty());

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
