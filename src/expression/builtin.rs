use std::ops::Not;

use super::elements::{Application, Lambda};
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
    /// "%"
    Remainder,
    Cond,
    /// `count`: count how many items
    /// in a list a given predicate returns
    /// true to
    Count,
}

impl BuiltIn {
    pub fn apply(
        self,
        mut args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        let arity_received = args.len() as _;

        match self {
            BuiltIn::Plus => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                Self::acc_numeric(|x, y| x + y, 0., expressions)
            }
            BuiltIn::Minus => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                Self::acc_numeric(|x, y| x - y, 0., expressions)
            }
            BuiltIn::Times => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                Self::acc_numeric(|x, y| x * y, 1., expressions)
            }
            BuiltIn::Divide => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                Self::acc_numeric(|x, y| x / y, 1., expressions)
            }
            BuiltIn::Equal => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                ensure_minimum_arity(2, arity_received)?;
                Self::equals(expressions)
            }
            BuiltIn::Not => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                // `not` can only be applied to one argument
                ensure_exact_arity(1, arity_received)?;
                Self::not(expressions)
            }
            BuiltIn::And => {
                let expressions = args
                    .into_iter()
                    .map(|expr| expr.evaluate(env));
                ensure_minimum_arity(2, arity_received)?;
                Self::and(expressions)
            }
            BuiltIn::Or => {
                ensure_minimum_arity(2, arity_received)?;

                Self::or(args, env)
            }
            BuiltIn::Remainder => {
                ensure_exact_arity(2, arity_received)?;

                // `pop`s will not fail since we've just checked
                // arity
                let rhs = args
                    .pop()
                    .unwrap()
                    .evaluate(env)?
                    .as_number()?;
                let lhs = args
                    .pop()
                    .unwrap()
                    .evaluate(env)?
                    .as_number()?;

                Ok((lhs % rhs).into())
            }
            BuiltIn::Count => {
                ensure_exact_arity(2, arity_received)?;

                let arguments = args
                    .pop()
                    .unwrap()
                    .evaluate(env)?
                    .as_list()?;

                let predicate = args
                    .pop()
                    .unwrap()
                    .evaluate(env)?
                    .as_lambda()?;

                Self::count(predicate, arguments)
            }
            BuiltIn::Cond => Self::cond(args, env),
        }
    }

    fn count(
        _predicate: Lambda,
        _arguments: Vec<Expression>,
    ) -> Result<Expression> {
        todo!()
    }

    fn cond(
        mut expressions: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        if expressions.is_empty() {
            return Ok(Expression::default());
        }

        // We have a default branch if this `cond` expression
        // has odd length
        let has_default_branch = expressions.len() % 2 == 1;
        // Safety: we've checked above that `expressions` is not
        // empty, therefore this will not fail
        let default_branch = has_default_branch
            .then(|| expressions.pop().unwrap());

        let mut expressions = expressions.into_iter();

        while let Some(condition) = expressions.next() {
            // TODO: convert to If here and evaluate it?
            let evaluated_cond = matches!(
                condition.evaluate(env)?,
                Expression::Atom(Atom::Boolean(true))
            );

            // Safety: we know `expressions` has even length,
            // therefore this unwrap won't fail
            let then = expressions.next().unwrap();
            if evaluated_cond {
                return then.evaluate(env);
            }
        }

        match default_branch {
            Some(default) => default.evaluate(env),
            None => {
                // Nothing evaluated to true, so we'll return nil
                Ok(Expression::default())
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
        expressions: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        for expression in expressions
            .into_iter()
            .map(|expr| expr.evaluate(env))
        {
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

        Ok(true.into())
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
