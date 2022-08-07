use std::ops::Not;

use crate::{
    ensure_arity, Atom, Env, Evaluable, Expression, Result,
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
            BuiltIn::Plus => {
                Self::acc_numeric(|x, y| x + y, 0., args, env)
            }
            BuiltIn::Minus => {
                Self::acc_numeric(|x, y| x - y, 0., args, env)
            }
            BuiltIn::Times => {
                Self::acc_numeric(|x, y| x * y, 1., args, env)
            }
            BuiltIn::Divide => {
                Self::acc_numeric(|x, y| x / y, 1., args, env)
            }
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
        let boolean = expr.evaluate(env)?.as_bool()?;

        Ok(Expression::Atom(Atom::Boolean(boolean.not())))
    }

    fn acc_numeric(
        func: impl Fn(f64, f64) -> f64,
        identity: f64,
        args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        let mut expressions = args
            .into_iter()
            .map(|expr| expr.evaluate(env))
            .peekable();

        let mut acc = match expressions.next() {
            Some(maybe_atom) => {
                let first_value = maybe_atom?.as_number()?;
                if expressions.peek().is_none() {
                    func(identity, first_value)
                } else {
                    first_value
                }
            }
            None => {
                return Ok(Expression::Atom(Atom::Number(0.0)))
            }
        };

        for expression in expressions {
            let expression = expression?;
            let number = expression.as_number()?;

            acc = func(acc, number)
        }

        Ok(Expression::Atom(Atom::Number(acc)))
    }
}
