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
    pub fn new<T>(string: T) -> FastString
        where T: AsRef<str> { // TODO Why?
        FastString(StringInner::new(string.as_ref()))
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

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

const SMALL_CAP: usize = max(size_of::<Arc<Vec<u8>>>(), 24) - 1;

#[derive(Clone, Debug)]
enum StringInner {
    Small { buf: [u8; SMALL_CAP], len: u8 },
    Large(Arc<Vec<u8>>),
}

impl StringInner {
    fn new(text: &str) -> Self {
        let len = text.len();
        return if len <= SMALL_CAP {
            let mut buf = [0; SMALL_CAP];
            buf[..len].copy_from_slice(text.as_bytes());
            StringInner::Small { buf, len: len as u8 }
        } else {
            StringInner::Large(Arc::from(Vec::from(text.as_bytes())))
        };
    }

    fn as_str(&self) -> &str {
        match self {
            StringInner::Small { buf, len } => {
                let len = *len as usize;
                let buf = &buf[..len];
                unsafe { from_utf8_unchecked(buf) } // TODO unsafe
            }
            StringInner::Large(data) => {
                let buf = &data[..data.len()];
                unsafe { from_utf8_unchecked(buf) } // TODO unsafe
            }
        }
    }

    fn push(&mut self, ch: char) {
        let mut temp = [0u8; 4];
        self.push_str(ch.encode_utf8(&mut temp));
    }

    // TODO Is this the optimal way?
    fn push_str(&mut self, string: &str) {
        match self {
            StringInner::Small { buf, len } => {
                let new_len = *len as usize + string.len();
                if new_len <= SMALL_CAP {
                    buf[*len as usize..new_len].copy_from_slice(string.as_bytes());
                    *len = new_len as u8
                } else {
                    let mut new_data = Vec::with_capacity(new_len);
                    new_data.extend_from_slice(&buf[..*len as usize]);
                    new_data.extend_from_slice(string.as_bytes());
                    *self = StringInner::Large(Arc::from(new_data)); // TODO ugly
                }
            }
            StringInner::Large(data) => {
                match Arc::get_mut(data) {
                    Some(old_data) => {
                        old_data.extend_from_slice(string.as_bytes());
                    }
                    None => {
                        let mut new_data: Vec<u8> = Vec::with_capacity(data.len() + string.len());
                        new_data.extend(data.iter());
                        new_data.extend_from_slice(string.as_bytes());
                        *self = StringInner::Large(Arc::from(new_data));
                    }
                }
            }
        };
    }
}