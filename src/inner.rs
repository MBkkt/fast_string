use std::mem::ManuallyDrop;
use std::{mem::size_of, ptr, slice, str::from_utf8_unchecked, sync::Arc, vec::Vec};

//               large len: 255 255 255 255
// little endian last byte: x..
//    big endian last byte:             ..x this is what we want
//                                      x.. but this is what we get if the code is the same for big endian
#[cfg(target_endian = "little")]
#[derive(Clone)]
#[repr(C)]
struct Large {
    data: Arc<Vec<u8>>,
    capacity: usize,
    len: usize,
}

// TODO this work only on little endian
const SMALL_CAPACITY: usize = size_of::<Large>() - 1;
const LARGE_FLAG: u8 = 0x80;
const LARGE_SHIFT: usize = (size_of::<usize>() - 1) * 8;
const LARGE_MASK: usize = !((LARGE_FLAG as usize) << LARGE_SHIFT);
const LARGE_BIT: usize = (LARGE_FLAG as usize) << LARGE_SHIFT;

#[derive(Clone, Copy)]
#[repr(C)]
struct Small {
    data: [u8; SMALL_CAPACITY],
    len: u8,
}

const _: () = [()][!(size_of::<Small>() == size_of::<Large>()) as usize];

#[repr(C)]
pub union StringInner {
    small: Small,
    large: ManuallyDrop<Large>,
}

impl Drop for StringInner {
    fn drop(&mut self) {
        unsafe {
            if self.is_large() {
                std::mem::drop(ManuallyDrop::take(&mut self.large));
            }
        }
    }
}

impl Clone for StringInner {
    fn clone(&self) -> Self {
        unsafe {
            if self.is_large() {
                Self {
                    large: self.large.clone(),
                }
            } else {
                Self { small: self.small }
            }
        }
    }
}

impl StringInner {
    pub fn new() -> Self {
        StringInner {
            small: Small {
                data: [0; SMALL_CAPACITY],
                len: 0,
            },
        }
    }

    pub fn from(text: &str) -> Self {
        let new_len = text.len();
        if new_len <= SMALL_CAPACITY {
            let mut new_data = [0; SMALL_CAPACITY];
            unsafe {
                ptr::copy_nonoverlapping(text.as_ptr(), new_data.as_mut_ptr(), new_len);
            }
            StringInner {
                small: Small {
                    data: new_data,
                    len: new_len as u8,
                },
            }
        } else {
            StringInner {
                large: ManuallyDrop::new(Large {
                    data: Arc::new(Vec::from(text)),
                    capacity: 0,
                    len: new_len | LARGE_BIT,
                }),
            }
        }
    }

    #[inline(always)]
    fn is_large(&self) -> bool {
        unsafe { (self.small.len & LARGE_FLAG) == LARGE_FLAG }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            from_utf8_unchecked(if self.is_large() {
                self.large.data.as_slice()
            } else {
                slice::from_raw_parts(self.small.data.as_ptr(), self.small.len as usize)
            })
        }
    }

    pub fn push_str(&mut self, string: &str) {
        unsafe {
            let str_len = string.len();
            if self.is_large() {
                match Arc::get_mut(&mut self.large.data) {
                    Some(old_data) => {
                        old_data.extend_from_slice(string.as_bytes());
                        self.large.len = old_data.len() | LARGE_BIT;
                    }
                    None => {
                        let new_len = self.large.data.len() + str_len;
                        let mut new_data = Vec::with_capacity(new_len);
                        new_data.extend_from_slice(self.large.data.as_slice());
                        new_data.extend_from_slice(string.as_bytes());
                        self.large.data = Arc::new(new_data);
                        self.large.len = new_len | LARGE_BIT;
                    }
                }
            } else {
                let old_len = self.small.len as usize;
                let new_len = old_len + str_len;
                if new_len <= SMALL_CAPACITY {
                    ptr::copy_nonoverlapping(
                        string.as_ptr(),
                        self.small.data.as_mut_ptr().add(old_len),
                        str_len,
                    );
                    self.small.len = new_len as u8;
                } else {
                    let mut new_data = Vec::with_capacity(new_len);
                    new_data.extend_from_slice(&self.small.data[..old_len]);
                    new_data.extend_from_slice(string.as_bytes());
                    *self = StringInner {
                        large: ManuallyDrop::new(Large {
                            data: Arc::new(new_data),
                            capacity: 0,
                            len: new_len | LARGE_BIT,
                        }),
                    };
                }
            }
        }
    }

    pub fn remove(&mut self, idx: usize) -> char {
        // TODO This isn't optimal, we check is_large twice, here and below:
        let ch = match self.as_str()[idx..].chars().next() {
            Some(ch) => ch,
            None => panic!("cannot remove a char from the end of a string"),
        };

        let len_ch = ch.len_utf8();
        let next = idx + len_ch;
        unsafe {
            if self.is_large() {
                let old_len = self.large.len & LARGE_MASK;
                let new_len = old_len - len_ch;
                self.large.len = new_len | LARGE_BIT;
                match Arc::get_mut(&mut self.large.data) {
                    Some(old_data) => {
                        old_data.drain(idx..next);
                    }
                    None => {
                        let mut new_data = Vec::with_capacity(new_len);
                        new_data.extend_from_slice(&self.large.data.as_slice()[..idx]);
                        new_data.extend_from_slice(&self.large.data.as_slice()[next..]);
                        self.large.data = Arc::new(new_data);
                    }
                }
            } else {
                let old_len = self.small.len as usize;
                ptr::copy(
                    self.small.data.as_ptr().add(next),
                    self.small.data.as_mut_ptr().add(idx),
                    old_len - next,
                );
                self.small.len = (old_len - len_ch) as u8;
            }
        }
        ch
    }
}
