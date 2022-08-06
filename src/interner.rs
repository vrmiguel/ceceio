#![allow(unused)]

use std::collections::HashSet;

use crate::{
    cheap_clone::CheapClone, small_string::SmallString,
};

pub struct StringInterner {
    inner: HashSet<SmallString>,
}

impl StringInterner {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashSet::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, element: &str) -> bool {
        self.inner.insert(SmallString::new(element))
    }

    pub fn get_or_insert(
        &mut self,
        element: &str,
    ) -> SmallString {
        match self.inner.get(element) {
            Some(small_str) => small_str.cheap_clone(),
            None => {
                let small_str = SmallString::new(element);
                self.inner.insert(small_str.cheap_clone());
                small_str
            }
        }
    }
}
