mod type_info;

pub use type_info::Typed;

use crate::{Error, Result};

fn nothing() {}

#[inline]
pub fn ensure_minimum_arity(
    at_least: u8,
    got: u8,
) -> Result<()> {
    (at_least <= got).then(nothing).ok_or(
        Error::MinimumArityMismatch {
            at_least,
            received: got,
        },
    )
}

#[inline]
pub fn ensure_exact_arity(want: u8, got: u8) -> Result<()> {
    (want == got).then(nothing).ok_or(
        Error::ExactArityMismatch {
            expected: want,
            received: got,
        },
    )
}
