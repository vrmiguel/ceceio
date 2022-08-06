use crate::{
    expression::{Application, Atom, If, IfElse},
    Expression, Result,
};

#[allow(unused)]
pub struct Evaluator {}

#[allow(unused)]
pub struct Env {}

pub trait Evaluable {
    fn evaluate(self, env: &mut Env) -> Result<Expression>;
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
    fn evaluate(self, _env: &mut Env) -> Result<Expression> {
        todo!()
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
        expression::Atom, parse_expression, Expression,
    };

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
}
