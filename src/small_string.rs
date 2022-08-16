use std::{
    borrow::Borrow, fmt, hash::Hash, ops::Deref, rc::Rc, str,
};

pub const INLINE_CAP: usize = 22;

#[derive(Clone, PartialEq, Eq)]
/// A cheaply-clonable String type
pub enum SmallString {
    Inlined { len: u8, buf: [u8; INLINE_CAP] },
    Heap(Rc<str>),
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for SmallString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SmallString::Inlined { len, buf } => {
                unsafe { buf.get_unchecked(0..*len as usize) }
                    .hash(state);
            }
            SmallString::Heap(rc) => {
                // Cold branch since identifiers tend to be
                // smaller than 23 bytes
                cold();
                rc.hash(state);
            }
        }

        #[cold]
        fn cold() {}
    }
}

impl fmt::Debug for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `s#` prefix -to flag that this is a SmallString
        write!(f, "s#\"{self}\"")
    }
}

impl fmt::Display for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SmallString {
    #[inline(always)]
    fn inlined(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() <= INLINE_CAP);
        let mut buf = [0u8; INLINE_CAP];

        // Safety: this function is internal and only called
        // after we've made sure that the given bytes are not
        // bigger than INLINE_CAP
        unsafe { buf.get_unchecked_mut(0..bytes.len()) }
            .copy_from_slice(bytes);
        Self::Inlined {
            len: bytes.len() as u8,
            buf,
        }
    }

    pub fn is_in_heap(&self) -> bool {
        matches!(self, Self::Heap(_))
    }

    pub fn new<S: AsRef<str>>(input: S) -> Self {
        let string = input.as_ref();
        let bytes = string.as_bytes();

        if bytes.len() > INLINE_CAP {
            Self::Heap(Rc::from(string))
        } else {
            Self::inlined(bytes)
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            // Safety: SmallString::Inlined can only be created
            // from `AsRef<str>`, so we'll
            // always have valid UTF-8
            SmallString::Inlined { buf, len } => unsafe {
                std::str::from_utf8_unchecked(
                    &buf[..*len as usize],
                )
            },
            SmallString::Heap(rc) => rc.as_ref(),
        }
    }
}

impl AsRef<str> for SmallString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for SmallString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for SmallString {
    fn borrow(&self) -> &str {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use super::SmallString;

    #[test]
    fn creates_inlined_small_strings_correctly() {
        let hey = SmallString::new("hey");
        assert_eq!(hey.as_str(), "hey");
        assert!(hey.is_in_heap().not());

        let length_22 =
            SmallString::new("abcdefghijkabcdefghijk");
        assert_eq!(length_22.as_str(), "abcdefghijkabcdefghijk");
        assert!(length_22.is_in_heap().not());

        let length_23 =
            SmallString::new("abcdefghijkabcdefghijkz");
        assert_eq!(
            length_23.as_str(),
            "abcdefghijkabcdefghijkz"
        );
        assert!(length_23.is_in_heap());
    }
}
