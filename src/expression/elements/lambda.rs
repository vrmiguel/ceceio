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
