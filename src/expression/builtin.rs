use crate::{
    Atom, Env, Error, Evaluable, Expression, Result, Typed,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Built-in operators
pub enum BuiltIn {
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Times,
    /// "/"
    Divide,
    /// "="
    Equal,
    /// "not"
    Not,
}

impl BuiltIn {
    pub fn apply(
        self,
        args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        match self {
            BuiltIn::Plus => Self::sum(args, env),
            BuiltIn::Minus => todo!(),
            BuiltIn::Times => todo!(),
            BuiltIn::Divide => todo!(),
            BuiltIn::Equal => todo!(),
            BuiltIn::Not => todo!(),
        }
    }

    fn sum(
        args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        let mut acc = 0.0;

        for expression in
            args.into_iter().map(|expr| expr.evaluate(env))
        {
            let expression = expression?;
            let number =
                expression.as_number().ok_or_else(|| {
                    Error::TypeMismatch {
                        expected: "number",
                        received: expression.rough_type(),
                    }
                })?;
            acc += number;
        }

        Ok(Expression::Atom(Atom::Number(acc)))
    }
}
