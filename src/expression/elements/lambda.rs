use super::Application;
use crate::{
    ensure_exact_arity, Atom, Env, Error, Evaluable, Expression,
    Result, SmallString,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub arguments: Vec<SmallString>,
    pub body: Expression,
}

impl Application {
    pub fn resolve_all(
        &mut self,
        fn_arguments: &[SmallString],
        received_arguments: &[Expression],
        env: &mut Env,
    ) -> Result<()> {
        for expression in self.arguments.iter_mut() {
            match expression {
                Expression::Atom(Atom::Identifier(
                    identifier,
                )) => {
                    // TODO: figure out a way of taking
                    // ownership of the argument instead of
                    // cloning
                    *expression = Lambda::resolve_argument(
                        identifier,
                        fn_arguments,
                        received_arguments,
                        env,
                    )?;
                }
                Expression::Application(app) => app
                    .resolve_all(
                        fn_arguments,
                        received_arguments,
                        env,
                    )?,
                _ => {}
            }
        }

        Ok(())
    }
}

impl Lambda {
    pub fn apply(
        self,
        mut received_arguments: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        // The code below is essentially the same as building a
        // local scope with the given arguments into
        // `env` and then calling `self.body.evaluate(env)`,
        // but allows us to better manage the arguments'
        // ownership, and should be generally faster
        // since we can avoid costly emulated frame stack
        // building and destruction, at the cost of
        // having big and weird code
        ensure_exact_arity(
            self.arguments.len() as _,
            received_arguments.len() as _,
        )?;

        match self.body {
            Expression::Atom(Atom::Identifier(identifier)) => {
                // TODO: check if identifier is in the outer
                // scope
                let idx = self
                    .arguments
                    .iter()
                    .position(|arg| arg == &identifier)
                    .ok_or(Error::UnknownSymbol(identifier))?;

                Ok(received_arguments.swap_remove(idx))
            }
            Expression::Atom(atom) => {
                // Other atoms are trivial, so we'll just return
                // them. E.g.: `(fn [] 3)`
                Ok(Expression::Atom(atom))
            }
            Expression::Application(mut app) => {
                // Resolve all identifiers (recursively, if there
                // are applications within this one)
                app.resolve_all(
                    &self.arguments,
                    &received_arguments,
                    env,
                )?;
                app.evaluate(env)
            }
            Expression::If(mut if_expr) => {
                if let Expression::Atom(Atom::Identifier(
                    ref identifier,
                )) = if_expr.condition
                {
                    if_expr.condition = Self::resolve_argument(
                        identifier,
                        &self.arguments,
                        &received_arguments,
                        env,
                    )?;
                }
                if let Expression::Atom(Atom::Identifier(
                    ref identifier,
                )) = if_expr.do_this
                {
                    if_expr.do_this = Self::resolve_argument(
                        identifier,
                        &self.arguments,
                        &received_arguments,
                        env,
                    )?;
                }

                if_expr.evaluate(env)
            }
            Expression::IfElse(_if_else_expr) => todo!(),
            Expression::Binding(binding) => {
                binding.evaluate(env)
            }
            Expression::Lambda(lambda) => {
                lambda.apply(received_arguments, env)
            }
        }
    }

    // TODO: transform this into a trait
    fn resolve_argument(
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
}

#[cfg(test)]
mod tests {
    use crate::Interpreter;

    #[test]
    fn evaluates_lambdas() {
        let mut interp = Interpreter::new();

        assert!(interp
            .parse_and_eval("(def ok (fn [] :ok))")
            .is_ok());

        assert_eq!(
            interp.parse_and_eval("(= (ok) :ok)").unwrap(),
            true.into()
        );

        assert!(interp
            .parse_and_eval("(def nothing (fn [x] nil))")
            .is_ok());

        assert_eq!(
            interp
                .parse_and_eval("(= (nothing 2) nil)")
                .unwrap(),
            true.into()
        );

        assert_eq!(
            interp
                .parse_and_eval(
                    "(= (nothing nothing) (nothing nil) nil)"
                )
                .unwrap(),
            true.into()
        );

        assert!(interp
            .parse_and_eval(
                "(def is-even? (fn [x] (= (% x 2) 0)))"
            )
            .is_ok());

        assert_eq!(
            interp.parse_and_eval("(is-even? 2)").unwrap(),
            true.into()
        );

        assert_eq!(
            interp.parse_and_eval("(is-even? 3)").unwrap(),
            false.into()
        );
    }

    #[test]
    fn subs_identifiers_by_their_values() {
        let mut interp = Interpreter::new();

        assert!(interp
            .parse_and_eval("(def id (fn [x] x)))")
            .is_ok());

        assert_eq!(
            interp
                .parse_and_eval("(= (id :success) :success)")
                .unwrap(),
            true.into()
        );

        assert!(interp
            .parse_and_eval("(def double (fn [x] (+ x x)))")
            .is_ok());

        assert_eq!(
            interp
                .parse_and_eval("(= (double 2) (* 4 1))")
                .unwrap(),
            true.into()
        );
    }
}
