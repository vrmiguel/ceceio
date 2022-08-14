#![allow(unused)]

use crate::{Atom, Error, Expression, Result, SmallString};

pub struct Scope<'a> {
    identifiers: &'a [SmallString],
    expressions: &'a [Expression],
}

impl Scope<'_> {
    pub fn resolve_symbol<'other>(
        &self,
        identifier_to_resolve: &'other SmallString,
    ) -> Result<&Expression> {
        let idx = self
            .identifiers
            .iter()
            .position(|identifier| {
                identifier == identifier_to_resolve
            })
            .ok_or_else(|| {
                Error::UnknownSymbol(
                    identifier_to_resolve.clone(),
                )
            })?;

        Ok(&self.expressions[idx])
    }

    pub fn resolve_all_in_place(
        &self,
        exprs: &mut [Expression],
    ) {
        for expression in exprs.iter_mut() {
            if let Expression::Atom(Atom::Identifier(
                identifier,
            )) = expression
            {
                if let Ok(resolved_expression) =
                    self.resolve_symbol(&*identifier)
                {
                    *expression = resolved_expression.clone();
                }
            }
        }
    }
}

impl<'a> Scope<'a> {
    pub fn new(
        identifiers: &'a [SmallString],
        expressions: &'a [Expression],
    ) -> Self {
        debug_assert_eq!(identifiers.len(), expressions.len());

        Self {
            identifiers,
            expressions,
        }
    }
}
