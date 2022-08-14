use std::{fmt, mem};

pub mod builtin;
pub mod elements;

pub use builtin::BuiltIn;

use self::elements::{
    Application, Atom, Binding, If, IfElse, Lambda,
};
use crate::{
    evaluatable::resolve_argument, Env, Error, Evaluable,
    Result, SmallString, Typed,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Application(Application),
    If(Box<If>),
    IfElse(Box<IfElse>),
    Binding(Box<Binding>),
    Lambda(Box<Lambda>),
}

impl Expression {
    pub fn as_number(&self) -> Result<f64> {
        if let Expression::Atom(Atom::Number(num)) = self {
            Ok(*num)
        } else {
            Err(Error::TypeMismatch {
                expected: "number",
                received: self.rough_type(),
            })
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if let Expression::Atom(Atom::Boolean(boolean)) = self {
            Ok(*boolean)
        } else {
            Err(Error::TypeMismatch {
                expected: "boolean",
                received: self.rough_type(),
            })
        }
    }

    pub fn as_lambda(&self) -> Result<&Lambda> {
        if let Expression::Lambda(lambda) = self {
            Ok(lambda)
        } else {
            Err(Error::TypeMismatch {
                expected: "lambda",
                received: self.rough_type(),
            })
        }
    }

    pub fn resolve_all(
        &mut self,
        fn_arguments: &[SmallString],
        received_arguments: &[Expression],
        env: &mut Env,
    ) -> Result<()> {
        match self {
            Expression::Atom(Atom::Identifier(identifier)) => {
                *self = resolve_argument(
                    identifier,
                    fn_arguments,
                    received_arguments,
                    env,
                )
                .or_else(|_| {
                    let expr = mem::take(self);
                    expr.evaluate(env)
                })?;
            }
            Expression::Application(app) => {
                for expression in app.arguments.iter_mut() {
                    expression.resolve_all(
                        fn_arguments,
                        received_arguments,
                        env,
                    )?
                }
            }
            Expression::If(if_expr) => {
                if_expr.condition.resolve_all(
                    fn_arguments,
                    received_arguments,
                    env,
                )?;

                if_expr.do_this.resolve_all(
                    fn_arguments,
                    received_arguments,
                    env,
                )?;
            }
            Expression::IfElse(if_else) => {
                if_else.condition.resolve_all(
                    fn_arguments,
                    received_arguments,
                    env,
                )?;

                if_else.if_true.resolve_all(
                    fn_arguments,
                    received_arguments,
                    env,
                )?;

                if_else.if_false.resolve_all(
                    fn_arguments,
                    received_arguments,
                    env,
                )?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl From<bool> for Expression {
    fn from(cond: bool) -> Self {
        Expression::Atom(Atom::Boolean(cond))
    }
}

impl From<f64> for Expression {
    fn from(number: f64) -> Self {
        Expression::Atom(Atom::Number(number))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Lambda(_) => f.write_str("<function>"),
            Expression::Atom(atom) => write!(f, "{atom}"),
            Expression::Application(app) => write!(f, "{app}"),
            Expression::Binding(binding) => {
                write!(f, "{binding}")
            }
            Expression::If(_) | Expression::IfElse(_) => {
                f.write_str("if")
            }
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Atom(Atom::Nil)
    }
}
