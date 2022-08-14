use std::fmt::Display;

mod lambda;

pub use lambda::Lambda;

use crate::{
    evaluatable::resolve_argument, BuiltIn, CheapClone, Env,
    Evaluable, Expression, Result, SmallString, Typed,
};

#[derive(Debug, Eq, PartialEq, Clone)]
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

/// Represents the binding of an
/// identifier to an expression
#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    pub identifier: SmallString,
    pub expression: Expression,
}

/// Represents an `if` predicate
#[derive(Debug, PartialEq, Clone)]
pub struct If {
    /// `if condition`
    pub condition: Expression,
    /// Then do this
    pub do_this: Expression,
}

impl If {
    pub fn resolve_identifiers_and_eval(
        mut self,
        fn_arguments: &[SmallString],
        received_arguments: &[Expression],
        env: &mut Env,
    ) -> Result<Expression> {
        if let Expression::Atom(Atom::Identifier(
            ref identifier,
        )) = self.condition
        {
            self.condition = resolve_argument(
                identifier,
                fn_arguments,
                received_arguments,
                env,
            )?;
        }
        if let Expression::Atom(Atom::Identifier(
            ref identifier,
        )) = self.do_this
        {
            self.do_this = resolve_argument(
                identifier,
                fn_arguments,
                received_arguments,
                env,
            )?;
        }

        self.evaluate(env)
    }
}

/// Represents an `if` predicate
/// with an `else` clause
#[derive(Debug, PartialEq, Clone)]
pub struct IfElse {
    pub condition: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

impl IfElse {
    pub fn resolve_identifiers_and_eval(
        mut self,
        fn_arguments: &[SmallString],
        received_arguments: &[Expression],
        env: &mut Env,
    ) -> Result<Expression> {
        println!("{self}");

        self.condition.resolve_all(
            fn_arguments,
            received_arguments,
            env,
        )?;

        self.if_true.resolve_all(
            fn_arguments,
            received_arguments,
            env,
        )?;

        self.if_false.resolve_all(
            fn_arguments,
            received_arguments,
            env,
        )?;

        dbg!(self.evaluate(env))
    }
}

// CheapClone since `SmallString` is
// cheap to clone and the rest is Copy
impl CheapClone for Atom {}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(f64),
    /// A symbol of the form `:symbol`
    Symbol(SmallString),
    // TODO: should identifier be in Expression?
    /// An identifier of a binding
    Identifier(SmallString),
    /// `true` or `false`
    Boolean(bool),
    BuiltIn(BuiltIn),
    String(SmallString),
    Nil,
}

impl Display for Atom {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Atom::Number(num) => write!(f, "{num}"),
            Atom::Symbol(symbol) => write!(f, ":{symbol}"),
            Atom::Boolean(boolean) => write!(f, "{boolean}"),
            Atom::String(string) => write!(f, "\"{string}\""),
            Atom::BuiltIn(built_in) => {
                write!(f, "<function {}>", built_in.rough_type())
            }
            Atom::Identifier(identifier) => {
                f.write_str(identifier)
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
        write!(f, "<function {}> ", self.name)
    }
}

impl Display for IfElse {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "(if {} {} {})",
            self.condition, self.if_true, self.if_false
        )
    }
}

impl Display for Binding {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "(def {} ({}))",
            self.identifier, self.expression
        )
    }
}
