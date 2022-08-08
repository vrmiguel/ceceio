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
    /// "or"
    Or,
}

impl BuiltIn {
    pub fn apply(
        self,
        args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        let arity_received = args.len() as _;
        let expressions =
            args.into_iter().map(|expr| expr.evaluate(env));

        match self {
            BuiltIn::Plus => {
                Self::acc_numeric(|x, y| x + y, 0., expressions)
            }
            BuiltIn::Minus => {
                Self::acc_numeric(|x, y| x - y, 0., expressions)
            }
            BuiltIn::Times => {
                Self::acc_numeric(|x, y| x * y, 1., expressions)
            }
            BuiltIn::Divide => {
                Self::acc_numeric(|x, y| x / y, 1., expressions)
            }
            BuiltIn::Equal => {
                ensure_minimum_arity(2, arity_received)?;
                Self::equals(expressions)
            }
            BuiltIn::Not => {
                // `not` can only be applied to one argument
                ensure_exact_arity(1, arity_received)?;
                Self::not(expressions)
            }
            BuiltIn::And => {
                ensure_minimum_arity(2, arity_received)?;
                Self::and(expressions)
            }
            BuiltIn::Or => {
                ensure_minimum_arity(2, arity_received)?;
                Self::or(expressions)
            }
        }
    }

    fn not(
        mut expressions: impl Iterator<Item = Result<Expression>>,
    ) -> Result<Expression> {
        // Won't fail because we've checked the arity in
        // Self::apply
        let expr = expressions.next().unwrap();
        debug_assert!(expressions.next().is_none());

        let boolean = expr?.as_bool()?;

        Ok(Expression::Atom(Atom::Boolean(boolean.not())))
    }

    fn and(
        expressions: impl Iterator<Item = Result<Expression>>,
    ) -> Result<Expression> {
        for expression in expressions {
            let expression = expression?;
            let condition = expression.as_bool()?;

            if condition.not() {
                return Ok(false.into());
            }
        }

        Ok(true.into())
    }

    fn or(
        expressions: impl Iterator<Item = Result<Expression>>,
    ) -> Result<Expression> {
        for expression in expressions {
            let expression = expression?;
            let condition = expression.as_bool()?;

            if condition {
                return Ok(true.into());
            }
        }

        Ok(false.into())
    }

    fn equals(
        mut expressions: impl Iterator<Item = Result<Expression>>,
    ) -> Result<Expression> {
        // Safe unwrap: minimum arity was checked in Self::apply
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
        expressions: impl Iterator<Item = Result<Expression>>,
    ) -> Result<Expression> {
        let mut expressions = expressions.peekable();

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
