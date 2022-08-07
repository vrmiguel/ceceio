use std::ops::Not;

use crate::{
    check::ensure_minimum_arity, ensure_exact_arity, Atom, Env,
    Evaluable, Expression, Result,
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
    /// "and"
    And,
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
            BuiltIn::Equal => Self::equals(args, env),
            BuiltIn::Not => Self::not(args, env),
            BuiltIn::And => Self::and(args, env),
        }
    }

    fn not(
        mut args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        // `not` can only be applied to one argument
        ensure_exact_arity(1, args.len() as _)?;

        // Won't fail because we've checked the arity above
        let expr = args.pop().unwrap();
        let boolean = expr.evaluate(env)?.as_bool()?;

        Ok(Expression::Atom(Atom::Boolean(boolean.not())))
    }

    fn and(
        args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        ensure_minimum_arity(2, args.len() as _)?;
        let expressions =
            args.into_iter().map(|expr| expr.evaluate(env));

        for expression in expressions {
            let expression = expression?;
            let condition = expression.as_bool()?;

            if condition.not() {
                return Ok(false.into());
            }
        }

        Ok(true.into())
    }

    fn equals(
        args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        ensure_minimum_arity(2, args.len() as _)?;

        let mut expressions =
            args.into_iter().map(|expr| expr.evaluate(env));

        // Safe unwrap: minimum arity was checked above
        let elem = expressions.next().unwrap()?;

        // Can't use iter::all due to error treatment
        for expression in expressions {
            let expression = expression?;
            if expression != elem {
                return Ok(false.into());
            }
        }

        return Ok(true.into());
    }

    /// Folds the argument list using the given function.
    ///
    /// `identity` is either the multiplicative identity or the
    /// additive identity (that is, 1 or 0), depending on the
    /// operation being used.
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
                    // If there are no more variables to fold,
                    // apply the only one we
                    // have to the identity and return
                    return Ok(
                        func(identity, first_value).into()
                    );
                }

                first_value
            }
            None => return Ok(0.0.into()),
        };

        for expression in expressions {
            let expression = expression?;
            let number = expression.as_number()?;

            acc = func(acc, number)
        }

        Ok(acc.into())
    }
}
