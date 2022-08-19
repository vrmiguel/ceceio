use std::rc::Rc;

use crate::{
    expression::elements::FnIdentifier,
    small_string::SmallString,
};

// Based on https://github.com/graphprotocol/graph-node/blob/master/graph/src/cheap_clone.rs
pub trait CheapClone: Clone {
    #[inline(always)]
    fn cheap_clone(&self) -> Self {
        self.clone()
    }
}

/// Cheap clone since amounts to a reference increment
impl<T: ?Sized> CheapClone for Rc<T> {}

/// Cheap clone since it amounts to either a memcpy of 24 stack
/// bytes or a reference increment
impl CheapClone for SmallString {}

impl CheapClone for FnIdentifier {}
