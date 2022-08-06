use crate::{CheapClone, SmallString};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Application(Application),
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
}

// CheapClone since `SmallString` is
// cheap to clone and the rest is Copy
impl CheapClone for Atom {}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(f64),
    /// A keyword of the form `:keyword`
    Keyword(SmallString),
    /// `true` or `false`
    Boolean(bool),
    // TODO: remove this from Atom since BuiltIn is
    // used in FnIdentifier?
    BuiltIn(BuiltIn),
}
