use std::fmt::Display;

use crate::{
    BuiltIn, CheapClone, Expression, SmallString, Typed,
};

#[derive(Debug, PartialEq, Clone)]
pub enum FnIdentifier {
    BuiltIn(BuiltIn),
    Other(SmallString),
}

#[derive(Debug, PartialEq, Clone)]
/// An application of the form `(name args*)`
pub struct Application {
    pub name: FnIdentifier,
    pub arguments: Vec<Expression>,
}

/// Represents an `if` predicate
#[derive(Debug, PartialEq, Clone)]
pub struct If {
    /// `if condition`
    pub condition: Expression,
    /// Then do this
    pub do_this: Expression,
}

/// Represents an `if` predicate
/// with an `else` clause
#[derive(Debug, PartialEq, Clone)]
pub struct IfElse {
    pub condition: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

// CheapClone since `SmallString` is
// cheap to clone and the rest is Copy
impl CheapClone for Atom {}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(f64),
    /// A keyword of the form `:keyword`
    Symbol(SmallString),
    /// `true` or `false`
    Boolean(bool),
    BuiltIn(BuiltIn),
    Nil,
}

impl Display for Atom {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Atom::Number(num) => write!(f, "{num}"),
            Atom::Symbol(keyword) => write!(f, ":{keyword}"),
            Atom::Boolean(boolean) => write!(f, "{boolean}"),
            Atom::BuiltIn(built_in) => {
                f.write_str(built_in.rough_type())
            }
            Atom::Nil => f.write_str("nil"),
        }
    }
}

impl Display for FnIdentifier {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            FnIdentifier::BuiltIn(built_in) => {
                f.write_str(built_in.rough_type())
            }
            FnIdentifier::Other(identifier) => {
                f.write_str(identifier)
            }
        }
    }
}

impl Display for Application {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
