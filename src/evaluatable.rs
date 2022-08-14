use std::collections::HashMap;

use crate::{
    expression::elements::{
        Application, Atom, Binding, FnIdentifier, If, IfElse,
    },
    Error, Expression, Result, SmallString,
};

#[allow(unused)]
#[derive(Debug, Default)]
pub struct Env {
    // Naive WIP representation
    bindings: HashMap<SmallString, Expression>,
}

impl Env {
    pub fn get(
        &self,
        identifier: SmallString,
    ) -> Result<Expression> {
        self.get_ref(identifier).map(Clone::clone)
    }

    pub fn get_ref(
        &self,
        identifier: SmallString,
    ) -> Result<&Expression> {
        self.bindings
            .get(&identifier)
            .ok_or(Error::UnknownSymbol(identifier))
    }
}

pub trait Evaluable {
    fn evaluate(self, env: &mut Env) -> Result<Expression>;
}

impl Evaluable for Atom {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        let expr = match self {
            Atom::Identifier(identifier) => {
                // Sub bindings by their value
                env.get(identifier)?
            }
            other => Expression::Atom(other),
        };

        Ok(expr)
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
            FnIdentifier::Other(identifier) => {
                let lambda = env
                    .get_ref(identifier)?
                    .as_lambda()?
                    .clone();

                lambda.apply(self.arguments, env)
            }
        }
    }
}

impl Evaluable for Binding {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        let expression = self.expression.evaluate(env)?;

        // We'll allow binding shadowing so whether or not
        // this binding previously existed is not important
        let _ = env
            .bindings
            .insert(self.identifier, expression.clone());

        Ok(expression)
    }
}

impl Evaluable for Expression {
    fn evaluate(self, env: &mut Env) -> Result<Expression> {
        match self {
            Expression::Lambda(lambda) => {
                Ok(Expression::Lambda(lambda))
            }
            Expression::Binding(binding) => {
                binding.evaluate(env)
            }
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

// TODO: transform this into a trait
pub fn resolve_argument(
    identifier: &SmallString,
    fn_arguments: &[SmallString],
    received_arguments: &[Expression],
    env: &mut Env,
) -> Result<Expression> {
    let idx = fn_arguments
        .iter()
        .position(|arg| arg == identifier)
        .ok_or_else(|| {
            Error::UnknownSymbol(identifier.clone())
        })?;

    received_arguments[idx].clone().evaluate(env)
}

#[cfg(test)]
mod tests {
    use super::{Env, Evaluable};
    use crate::{
        expression::elements::Atom, parse_expression, Error,
        Expression, Interpreter, Result,
    };

    // TODO: finish converting test cases to use `parse_and_eval`
    fn parse_and_eval(input: &str) -> Result<Expression> {
        parse_and_eval_with_env(input, &mut Env::default())
    }

    fn parse_and_eval_with_env(
        input: &str,
        env: &mut Env,
    ) -> Result<Expression> {
        let expr = parse_expression(input).unwrap().1;
        expr.evaluate(env)
    }

    #[test]
    fn evaluates_bindings() {
        let mut interp = Interpreter::new();
        assert!(parse_expression("(def)").is_err());
        assert!(parse_expression("(def x)").is_err());
        assert!(parse_expression("(def 2 2)").is_err());

        assert_eq!(
            interp.parse_and_eval("(def five 5.0)").unwrap(),
            Expression::Atom(Atom::Number(5.0))
        );

        assert_eq!(
            // Ensure that we can retrieve previous bindings
            // from our env
            interp
                .parse_and_eval("(def six (+ five 1.0))",)
                .unwrap(),
            Expression::Atom(Atom::Number(6.0))
        );
    }

    #[test]
    fn evaluates_equality_correctly() {
        // Must fail arity check
        assert!(parse_and_eval("(=)").is_err());
        assert!(parse_and_eval("(= 2)").is_err());

        assert_eq!(
            parse_and_eval("(= 2 2)").unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval("(= 2 3)").unwrap(),
            false.into()
        );
        assert_eq!(
            parse_and_eval("(= (+ 1 3) (+ 2 2) (- 6 2))")
                .unwrap(),
            true.into()
        );
    }

    #[test]
    fn evaluates_and_operator_correctly() {
        // Must fail arity check
        assert!(parse_and_eval("(and)").is_err());
        assert!(parse_and_eval("(and 2)").is_err());

        // Must fail type check
        assert_eq!(
            parse_and_eval("(and 2 2)").unwrap_err(),
            Error::TypeMismatch {
                expected: "boolean",
                received: "number"
            }
        );

        assert_eq!(
            parse_and_eval("(and true true)").unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval("(and false true)").unwrap(),
            false.into()
        );
        assert_eq!(parse_and_eval("(and true true true true true true true true false true)").unwrap(), false.into());
        assert_eq!(
            parse_and_eval(
                "(and (= 2 2) (= 3 3) (= (= 2 5) (=7 8))))"
            )
            .unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval(
                "(and (= 2 2) (= 2 3) (= (= 2 5) (=7 8))))"
            )
            .unwrap(),
            false.into()
        );
    }

    #[test]
    fn evaluates_or_operator_correctly() {
        // Must fail arity check
        assert!(parse_and_eval("(or)").is_err());
        assert!(parse_and_eval("(or 2)").is_err());

        // Must fail type check
        assert_eq!(
            parse_and_eval("(or 2 true)").unwrap_err(),
            Error::TypeMismatch {
                expected: "boolean",
                received: "number"
            }
        );

        assert_eq!(
            parse_and_eval("(or true true)").unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval("(or false true)").unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval(
                "(or false false false false false true)"
            )
            .unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval(
                "(or (= 2 2) (= 3 3) (= (= 2 5) (=7 8))))"
            )
            .unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval(
                "(or (= 2 3) (= 3 3) (= (= 2 5) (=7 8))))"
            )
            .unwrap(),
            true.into()
        );
        assert_eq!(
            parse_and_eval(
                "(or (= 2 3) (= 5 3) (= (= 2 5) (=7 8))))"
            )
            .unwrap(),
            true.into()
        );
    }

    #[test]
    fn evaluates_addition_correctly() {
        assert_eq!(parse_and_eval("(+)").unwrap(), 0.0.into());

        assert_eq!(parse_and_eval("(+ 3)").unwrap(), 3.0.into());

        assert_eq!(parse_and_eval("(+ 5)").unwrap(), 5.0.into());

        assert_eq!(
            parse_and_eval("(+ (+ 3 5) (+ (if true 5 2) 2))")
                .unwrap(),
            15.0.into()
        );
    }

    #[test]
    fn evaluates_remainder_operations() {
        let mut interp = Interpreter::new();
        assert_eq!(
            interp.parse_and_eval("(% 5 2)").unwrap(),
            1.0.into()
        );
        assert_eq!(
            interp.parse_and_eval("(= (% 4 2) 0)").unwrap(),
            true.into()
        );
        assert_eq!(
            interp
                .parse_and_eval("(= (% 4 2) (% 9 3) (% 33 11))")
                .unwrap(),
            true.into()
        );

        // Must fail arity check
        interp.parse_and_eval("(%)").unwrap_err();
        interp.parse_and_eval("(% 1)").unwrap_err();
        interp.parse_and_eval("(% 1 4 5)").unwrap_err();

        // Must fail type-check
        interp.parse_and_eval("(% :ok :hey)").unwrap_err();
    }

    #[test]
    fn evaluates_multiplication_correctly() {
        let expr = parse_expression("(*)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );

        let expr = parse_expression("(* 3)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(3.0))
        );

        let expr = parse_expression("(* 3 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(6.0))
        );

        let expr = parse_expression("(* 3 2 1)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(6.0))
        );

        let expr = parse_expression("(* 3 2 1 0)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );
    }

    #[test]
    fn evaluates_subtraction_correctly() {
        let expr = parse_expression("(-)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(0.0))
        );

        let expr = parse_expression("(- 3)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(-3.0))
        );

        // FIXME: this is wrong
        let expr = parse_expression("(- 3 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(1.0))
        );
    }

    #[test]
    fn evaluates_if_expressions() {
        let expr = parse_expression("(if true 2)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(2.0))
        );

        let expr = parse_expression("(if false 2 (if true 5))")
            .unwrap()
            .1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Number(5.0))
        );
    }

    #[test]
    fn evaluates_not_expressions() {
        let expr = parse_expression("(not true)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Boolean(false))
        );

        let expr = parse_expression("(not false)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Boolean(true))
        );

        let expr = parse_expression("(not (not (not false)))")
            .unwrap()
            .1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap(),
            Expression::Atom(Atom::Boolean(true))
        );

        let expr = parse_expression("(not)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap_err(),
            Error::ExactArityMismatch {
                expected: 1,
                received: 0
            }
        );

        let expr =
            parse_expression("(not false true)").unwrap().1;
        assert_eq!(
            expr.evaluate(&mut Env::default()).unwrap_err(),
            Error::ExactArityMismatch {
                expected: 1,
                received: 2
            }
        );
    }
}
