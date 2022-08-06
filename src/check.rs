mod type_info;

pub use type_info::Typed;

use crate::{Error, Result};

pub fn ensure_arity(want: u8, got: u8) -> Result<()> {
    (want == got).then(|| {}).ok_or(Error::ArityMismatch {
        expected: want,
        received: got,
    })
}
