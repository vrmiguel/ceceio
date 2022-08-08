use crate::{Atom, BuiltIn, Expression};

pub trait Typed {
    /// Displays roughly what type this expression or atom is,
    /// for error message purposes
    fn rough_type(&self) -> &'static str;
}

impl Typed for Expression {
    fn rough_type(&self) -> &'static str {
        match self {
            Expression::Binding(_) => "binding",
            Expression::Atom(atom) => atom.rough_type(),
            Expression::Application(_) => "application",
            Expression::If(_) | Expression::IfElse(_) => "if",
        }
    }
}

impl Typed for BuiltIn {
    fn rough_type(&self) -> &'static str {
        match self {
            BuiltIn::Plus => "+",
            BuiltIn::Minus => "-",
            BuiltIn::Times => "*",
            BuiltIn::Divide => "/",
            BuiltIn::Equal => "=",
            BuiltIn::Not => "not",
            BuiltIn::And => "and",
            BuiltIn::Or => "or",
        }
    }
}

impl Typed for Atom {
    fn rough_type(&self) -> &'static str {
        match self {
            Atom::Number(_) => "number",
            Atom::Identifier(_) => "identifier",
            Atom::Symbol(_) => "symbol",
            Atom::Boolean(boolean) => {
                if *boolean {
                    "true"
                } else {
                    "false"
                }
            }
            Atom::BuiltIn(built_in) => built_in.rough_type(),
            Atom::Nil => "nil",
        }
    }
}
