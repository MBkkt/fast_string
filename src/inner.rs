use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{mem::size_of, ptr, slice, str::from_utf8_unchecked};

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

const CACHE_LINE_SIZE: usize = max(64 /* maybe 128? */, size_of::<AtomicUsize>());
// TODO make power of two

struct ArcVecU8 {
    data: *mut u8,
}

impl ArcVecU8 {
    pub unsafe fn with_capacity(capacity: usize) -> Self {
        let pointer = alloc(Layout::from_size_align_unchecked(
            CACHE_LINE_SIZE + capacity,
            CACHE_LINE_SIZE,
        ));
        ptr::write(pointer as *mut AtomicUsize, AtomicUsize::new(1));
        Self {
            data: pointer.add(CACHE_LINE_SIZE),
        }
    }

    #[inline(never)]
    unsafe fn drop_slow(&mut self, capacity: usize) {
        std::sync::atomic::fence(Ordering::Acquire);
        dealloc(
            self.data.sub(CACHE_LINE_SIZE),
            Layout::from_size_align_unchecked(CACHE_LINE_SIZE + capacity, CACHE_LINE_SIZE),
        );
    }

    #[inline]
    pub unsafe fn drop(&mut self, capacity: usize) {
        if self.get_counter().fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        self.drop_slow(capacity);
    }

    pub unsafe fn get_mut(&mut self) -> Option<&mut ArcVecU8> {
        if self.get_counter().load(Ordering::Acquire) == 1 {
            Some(self)
        } else {
            None
        }
    }

    pub unsafe fn reserve(&mut self, old_capacity: usize, new_capacity: usize) {
        // TODO old_capacity should be old_size or old_capacity?
        //      If capacity its strange, I want try_realloc and if null,
        //      alloc new_capacity and copy only old_size
        self.data = realloc(
            self.data.sub(CACHE_LINE_SIZE),
            Layout::from_size_align_unchecked(CACHE_LINE_SIZE + old_capacity, CACHE_LINE_SIZE),
            CACHE_LINE_SIZE + new_capacity,
        )
        .add(CACHE_LINE_SIZE);
    }

    pub unsafe fn extend_from(&mut self, old_size: usize, bytes: *const u8, size: usize) {
        ptr::copy_nonoverlapping(bytes, self.data.add(old_size), size);
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data
    }

    #[inline(always)]
    unsafe fn get_counter(&self) -> &AtomicUsize {
        &*(self.data.sub(CACHE_LINE_SIZE) as *mut AtomicUsize)
    }

    #[inline(always)]
    pub fn align_capacity(capacity: usize) -> usize {
        ((capacity + CACHE_LINE_SIZE - 1) / CACHE_LINE_SIZE) * CACHE_LINE_SIZE
    }
}

impl Clone for ArcVecU8 {
    fn clone(&self) -> Self {
        unsafe {
            self.get_counter().fetch_add(1, Ordering::Relaxed);
        }
        Self { data: self.data }
    }
}

//               large len: 255 255 255 255
// little endian last byte: x..
//    big endian last byte:             ..x this is what we want
//                                      x.. but this is what we get if the code is the same for big endian
#[cfg(target_endian = "little")]
#[derive(Clone)]
#[repr(C)]
struct Large {
    data: ArcVecU8,
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
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.is_large() {
                let mut large = ManuallyDrop::take(&mut self.large);
                large.data.drop(large.capacity);
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
        unsafe {
            if new_len <= SMALL_CAPACITY {
                let mut new_data = [0; SMALL_CAPACITY];
                ptr::copy_nonoverlapping(text.as_ptr(), new_data.as_mut_ptr(), new_len);
                Self {
                    small: Small {
                        data: new_data,
                        len: new_len as u8,
                    },
                }
            } else {
                let new_capacity = ArcVecU8::align_capacity(new_len);
                let mut new_data = ArcVecU8::with_capacity(new_capacity);
                new_data.extend_from(0, text.as_ptr(), new_len);
                Self {
                    large: ManuallyDrop::new(Large {
                        data: new_data,
                        capacity: new_capacity,
                        len: new_len | LARGE_BIT,
                    }),
                }
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
                slice::from_raw_parts(self.large.data.as_ptr(), self.large.len & LARGE_MASK)
            } else {
                slice::from_raw_parts(self.small.data.as_ptr(), self.small.len as usize)
            })
        }
    }

    pub fn push_str(&mut self, string: &str) {
        let str_len = string.len();
        unsafe {
            if self.is_large() {
                let old_len = self.large.len & LARGE_MASK;
                let new_len = old_len + str_len;
                let mut capacity = self.large.capacity;
                match self.large.data.get_mut() {
                    Some(old_data) => {
                        if capacity < new_len {
                            let new_capacity =
                                ArcVecU8::align_capacity(std::cmp::max(new_len, capacity * 3 / 2));
                            old_data.reserve(capacity, new_capacity);
                            capacity = new_capacity;
                        }
                        old_data.extend_from(old_len, string.as_ptr(), str_len);
                    }
                    None => {
                        let new_capacity = ArcVecU8::align_capacity(new_len);
                        let mut new_data = ArcVecU8::with_capacity(new_capacity);
                        new_data.extend_from(0, self.large.data.as_ptr(), old_len);
                        new_data.extend_from(old_len, string.as_ptr(), str_len);
                        self.large.data.drop(capacity);
                        self.large.data = new_data;
                        capacity = new_capacity;
                    }
                }
                self.large.capacity = capacity;
                self.large.len = new_len | LARGE_BIT;
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
                    let new_capacity = ArcVecU8::align_capacity(new_len);
                    let mut new_data = ArcVecU8::with_capacity(new_capacity);
                    new_data.extend_from(0, self.small.data.as_ptr(), old_len);
                    new_data.extend_from(old_len, string.as_ptr(), str_len);
                    *self = StringInner {
                        large: ManuallyDrop::new(Large {
                            data: new_data,
                            capacity: new_capacity,
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
                match self.large.data.get_mut() {
                    Some(old_data) => {
                        ptr::copy(
                            old_data.as_ptr().add(next),
                            old_data.as_mut_ptr().add(idx),
                            old_len - next,
                        );
                    }
                    None => {
                        let new_capacity = ArcVecU8::align_capacity(new_len);
                        let mut new_data = ArcVecU8::with_capacity(new_capacity);
                        new_data.extend_from(0, self.large.data.as_ptr(), idx);
                        new_data.extend_from(
                            idx,
                            self.large.data.as_ptr().add(next),
                            old_len - next,
                        );
                        let old_capacity = self.large.capacity;
                        self.large.data.drop(old_capacity);
                        self.large.data = new_data;
                        self.large.capacity = new_capacity;
                    }
                }
                self.large.len = new_len | LARGE_BIT;
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
