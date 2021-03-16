use std::{
    sync::Arc,
    mem::size_of,
    str::from_utf8_unchecked,
    vec::Vec,
    fmt,
    cmp::Ord,
    cmp::Ordering,
    hash,
};

#[derive(Clone)]
pub struct FastString(StringInner);

impl FastString {
    #[inline(always)]
    pub fn new<T>(text: T) -> FastString
        where T: AsRef<str> {
        FastString(StringInner::new(text))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}


impl fmt::Debug for FastString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for FastString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl PartialEq<FastString> for FastString {
    fn eq(&self, other: &FastString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for FastString {}

impl PartialEq<str> for FastString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<FastString> for str {
    fn eq(&self, other: &FastString) -> bool {
        other == self
    }
}

impl<'a> PartialEq<&'a str> for FastString {
    fn eq(&self, other: &&'a str) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<FastString> for &'a str {
    fn eq(&self, other: &FastString) -> bool {
        other == *self
    }
}

impl PartialEq<String> for FastString {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<FastString> for String {
    fn eq(&self, other: &FastString) -> bool {
        other == self
    }
}

impl<'a> PartialEq<&'a String> for FastString {
    fn eq(&self, other: &&'a String) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<FastString> for &'a String {
    fn eq(&self, other: &FastString) -> bool {
        *self == other
    }
}

impl Ord for FastString {
    fn cmp(&self, other: &FastString) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for FastString {
    fn partial_cmp(&self, other: &FastString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for FastString {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

const SMALL_CAP: usize = max(size_of::<Vec<u8>>(), size_of::<Arc<[u8]>>()) - 1;

const MEDIUM_CAP: usize = 256;

assert!(SMALL_CAP < MEDIUM_CAP);

#[derive(Clone, Debug)]
enum StringInner {
    Small { buf: [u8; SMALL_CAP], len: u8 },
    Medium(Vec<u8>),
    Large(Arc<[u8]>),
}

impl StringInner {
    fn new<T>(text: T) -> Self
        where T: AsRef<str> {
        let text = text.as_ref();
        let len = text.len();
        return if len <= SMALL_CAP {
            let mut buf = [0; SMALL_CAP];
            buf[..len].copy_from_slice(text.as_bytes());
            StringInner::Small { buf, len: len as u8 }
        } else {
            let mut buf = Vec::from(text.as_bytes());
            if len <= MEDIUM_CAP {
                StringInner::Medium(buf.into())
            } else {
                StringInner::Large(buf.into())
            }
        };
    }

    #[inline(always)]
    fn len(&self) -> usize {
        match self {
            StringInner::Small { len, .. } => *len as usize,
            StringInner::Medium(data) => data.len(),
            StringInner::Large(data) => data.len(),
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        match self {
            StringInner::Small { buf, len } => {
                let len = *len as usize;
                let buf = &buf[..len];
                unsafe { from_utf8_unchecked(buf) }
            }
            StringInner::Medium(data) => {
                let buf = &data[..data.len()];
                unsafe { from_utf8_unchecked(buf) }
            }
            StringInner::Large(data) => {
                let buf = &data[..data.len()];
                unsafe { from_utf8_unchecked(buf) }
            }
        }
    }
}