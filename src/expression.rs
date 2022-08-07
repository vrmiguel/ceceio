pub mod builtin;
pub mod elements;

pub use builtin::BuiltIn;

use self::elements::{Application, Atom, If, IfElse};
use crate::{Error, Result, Typed};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Application(Application),
    If(Box<If>),
    IfElse(Box<IfElse>),
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
