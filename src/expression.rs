use crate::{CheapClone, SmallString};

#[derive(Debug, PartialEq)]
pub enum FnIdentifier {
    BuiltIn(BuiltIn),
    Other(SmallString),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    Not,
}

impl CheapClone for Atom {}
#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(f64),
    Keyword(SmallString),
    Boolean(bool),
    BuiltIn(BuiltIn),
}
