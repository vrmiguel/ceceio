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

impl Lambda {
    pub fn apply(
        self,
        mut args: Vec<Expression>,
        env: &mut Env,
    ) -> Result<Expression> {
        // The code below is essentially the same as building a
        // local scope with the given arguments into
        // `env` and then calling `self.body.evaluate(env)`,
        // but allows us to better manage the arguments'
        // ownership
        ensure_exact_arity(
            self.arguments.len() as _,
            args.len() as _,
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

                Ok(args.swap_remove(idx))
            }
            Expression::Atom(atom) => {
                // Other atoms are trivial, so we'll just return
                // them. E.g.: `(fn [] 3)`
                Ok(Expression::Atom(atom))
            }
            Expression::Application(app) => {
                let Application {
                    name,
                    mut arguments,
                } = app;
                for expression in arguments.iter_mut() {
                    if let Expression::Atom(Atom::Identifier(
                        identifier,
                    )) = expression
                    {
                        let idx = self
                            .arguments
                            .iter()
                            .position(|arg| arg == &*identifier)
                            .ok_or_else(|| {
                                Error::UnknownSymbol(
                                    identifier.clone(),
                                )
                            })?;

                        // TODO: figure out a way of taking
                        // ownership of the argument instead of
                        // cloning
                        *expression = args[idx].clone();
                    }
                }

                Application { name, arguments }.evaluate(env)
            }
            Expression::If(_) => todo!(),
            Expression::IfElse(_) => todo!(),
            Expression::Binding(binding) => {
                binding.evaluate(env)
            }
            Expression::Lambda(lambda) => {
                lambda.apply(args, env)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Interpreter;

    #[test]
    fn evaluates_atomic_lambdas() {
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
            interp.parse_and_eval("(= (nothing) nil)").unwrap(),
            true.into()
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
