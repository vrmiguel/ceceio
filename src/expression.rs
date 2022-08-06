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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(Atom),
    Application(Application),
    If(Box<If>),
    IfElse(Box<IfElse>),
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
    Nil,
}
