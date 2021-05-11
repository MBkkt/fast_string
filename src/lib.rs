mod inner;

use inner::StringInner;
use std::{cmp::Ord, cmp::Ordering, fmt, hash};

#[derive(Clone)]
pub struct FastString(StringInner);

impl FastString {
    pub fn new() -> Self {
        Self(StringInner::new())
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline(always)]
    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    #[inline(always)]
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string);
    }

    #[inline(always)]
    pub fn remove(&mut self, idx: usize) -> char {
        self.0.remove(idx)
    }
}

impl Default for FastString {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> From<&'a str> for FastString {
    fn from(string: &str) -> Self {
        Self(StringInner::from(string))
    }
}

impl<'a> From<&'a mut str> for FastString {
    fn from(string: &mut str) -> Self {
        Self(StringInner::from(string))
    }
}

impl From<String> for FastString {
    fn from(string: String) -> Self {
        Self(StringInner::from(string.as_str()))
    }
}

impl From<FastString> for String {
    fn from(string: FastString) -> Self {
        String::from(string.as_str())
    }
}

impl From<char> for FastString {
    fn from(ch: char) -> Self {
        let mut temp = [0u8; 4];
        Self(StringInner::from(ch.encode_utf8(&mut temp)))
    }
}

impl std::ops::Deref for FastString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
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
