use crate::{Atom, BuiltIn, Expression};

pub trait Typed {
    /// Displays roughly what type this expression or atom is
    fn rough_type(&self) -> &'static str;
}

impl Typed for Expression {
    fn rough_type(&self) -> &'static str {
        match self {
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
        }
    }
}

impl Typed for Atom {
    fn rough_type(&self) -> &'static str {
        match self {
            Atom::Number(_) => "Number",
            Atom::Keyword(_) => "Keyword",
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
