use crate::{
    ensure_exact_arity, Atom, Env, Evaluable, Expression,
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
        args: Vec<Expression>,
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
            }
            Expression::Atom(atom) => {
                // Other atoms are trivial, so we'll just return
                // them. E.g.: `(fn [] 3)`
                return Ok(Expression::Atom(atom));
            }
            Expression::Application(_) => todo!(),
            Expression::If(_) => todo!(),
            Expression::IfElse(_) => todo!(),
            Expression::Binding(binding) => {
                return binding.evaluate(env)
            }
            Expression::Lambda(lambda) => {
                return lambda.apply(args, env)
            }
        }

        // let expressions =
        //     args.into_iter().map(|expr| expr.evaluate(env));
        todo!()
    }
}
