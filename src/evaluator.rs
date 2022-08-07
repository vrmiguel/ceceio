use crate::{
    expression::{
        elements::{
            Application, Atom, FnIdentifier, If, IfElse,
        },
        BuiltIn,
    },
    Expression, Result,
};

#[allow(unused)]
pub struct Evaluator {}

#[allow(unused)]
pub struct Env {}

pub trait Evaluable {
    fn evaluate(self, env: &mut Env) -> Result<Expression>;
}

impl Evaluable for BuiltIn {
    fn evaluate(self, _: &mut Env) -> Result<Expression> {
        match self {
            BuiltIn::Plus => todo!(),
            BuiltIn::Minus => todo!(),
            BuiltIn::Times => todo!(),
            BuiltIn::Divide => todo!(),
            BuiltIn::Equal => todo!(),
            BuiltIn::Not => todo!(),
        }
    }
}

impl Evaluable for Atom {
    fn evaluate(self, _: &mut Env) -> Result<Expression> {
        // No further processing necessary
        Ok(Expression::Atom(self))
    }
}

impl Evaluable for If {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        let cond = matches!(
            self.condition.evaluate(env)?,
            Expression::Atom(Atom::Boolean(true))
        );
        if cond {
            self.do_this.evaluate(env)
        } else {
            Ok(Expression::Atom(Atom::Nil))
        }
    }
}

impl Evaluable for IfElse {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        let cond = matches!(
            self.condition.evaluate(env)?,
            Expression::Atom(Atom::Boolean(true))
        );
        if cond {
            self.if_true.evaluate(env)
        } else {
            self.if_false.evaluate(env)
        }
    }
}

impl Evaluable for Application {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        match self.name {
            FnIdentifier::BuiltIn(built_in) => {
                built_in.apply(self.arguments, env)
            }
            FnIdentifier::Other(_) => todo!(),
        }
    }
}

impl Evaluable for Expression {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        match self {
            Expression::Atom(atom) => atom.evaluate(env),
            Expression::Application(application) => {
                application.evaluate(env)
            }
            Expression::If(if_expr) => if_expr.evaluate(env),
            Expression::IfElse(if_else_expr) => {
                if_else_expr.evaluate(env)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Env, Evaluable};
    use crate::{
        expression::elements::Atom, parse_expression, Error,
        Expression,
    };

    #[test]
    fn evaluates_addition_correctly() {
        let expr = parse_expression("(+)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );

        let expr = parse_expression("(+ 3)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(3.0))
        );

        let expr = parse_expression("(+ 3 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(5.0))
        );

        let expr =
            parse_expression("(+ (+ 3 5) (+ (if true 5 2) 2))")
                .unwrap()
                .1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(15.0))
        );
    }

    #[test]
    fn evaluates_multiplication_correctly() {
        let expr = parse_expression("(*)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );

        let expr = parse_expression("(* 3)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(3.0))
        );

        let expr = parse_expression("(* 3 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(6.0))
        );

        let expr = parse_expression("(* 3 2 1)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(6.0))
        );

        let expr = parse_expression("(* 3 2 1 0)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );
    }

    #[test]
    fn evaluates_subtraction_correctly() {
        let expr = parse_expression("(-)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );

        let expr = parse_expression("(- 3)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(-3.0))
        );

        // FIXME: this is wrong
        let expr = parse_expression("(- 3 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(1.0))
        );
    }

    #[test]
    fn evaluates_if_expressions() {
        let expr = parse_expression("(if true 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(2.0))
        );

        let expr = parse_expression("(if false 2 (if true 5))")
            .unwrap()
            .1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Number(5.0))
        );
    }

    #[test]
    fn evaluates_not_expressions() {
        let expr = parse_expression("(not true)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Boolean(false))
        );

        let expr = parse_expression("(not false)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Boolean(true))
        );

        let expr = parse_expression("(not (not (not false)))")
            .unwrap()
            .1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap(),
            Expression::Atom(Atom::Boolean(true))
        );

        let expr = parse_expression("(not)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap_err(),
            Error::ArityMismatch {
                expected: 1,
                received: 0
            }
        );

        let expr =
            parse_expression("(not false true)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env {}).unwrap_err(),
            Error::ArityMismatch {
                expected: 1,
                received: 2
            }
        );
    }
}
