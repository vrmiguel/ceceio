pub mod builtin;
pub mod elements;

pub use builtin::BuiltIn;

use self::elements::{Application, Atom, If, IfElse};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Application(Application),
    If(Box<If>),
    IfElse(Box<IfElse>),
}

impl Expression {
    pub fn as_number(&self) -> Option<f64> {
        if let Expression::Atom(Atom::Number(num)) = self {
            Some(*num)
        } else {
            None
        }
    }
}
