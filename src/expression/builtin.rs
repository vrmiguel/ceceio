use std::ops::Not;

use crate::{
    ensure_arity, Atom, Env, Error, Evaluable, Expression,
    Result, Typed,
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
            BuiltIn::Not => Self::not(args, env),
        }
    }

    fn not(
        mut args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        ensure_arity(1, args.len() as _)?;

        // Won't fail because we've checked the arity above
        let expr = args.pop().unwrap();
        let expr = expr.evaluate(env)?;

        let boolean = expr.as_bool().ok_or_else(|| {
            Error::TypeMismatch {
                expected: "bool",
                received: expr.rough_type(),
            }
        })?;

        Ok(Expression::Atom(Atom::Boolean(boolean.not())))
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
