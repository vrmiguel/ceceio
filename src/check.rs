mod type_info;

pub use type_info::Typed;

use crate::{Error, Result};

#[inline(always)]
pub fn ensure_minimum_arity(at_least: u8, got: u8) -> Result<()> {
    (at_least <= got).then(|| {}).ok_or(Error::MinimumArityMismatch { at_least, received: got })
}

pub fn ensure_exact_arity(want: u8, got: u8) -> Result<()> {
    (want == got).then(|| {}).ok_or(Error::ExactArityMismatch {
        expected: want,
        received: got,
    })
}
