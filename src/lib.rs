use std::mem;
use std::ops::{
    Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive,
};
use std::slice::SliceIndex;

pub struct RightPadString {
    s: String,
}

impl RightPadString {
    pub fn push(&mut self, ch: char) {
        let _ = self.s.pop();
        self.s.push(ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        let _ = self.s.pop();
        let ch = self.s.pop();
        self.s.push('\0');
        ch
    }

    pub fn as_str(&self) -> &str {
        let total_len = self.s.len();
        &self.s[0..total_len - 1]
    }
}

impl From<String> for RightPadString {
    fn from(mut s: String) -> Self {
        s.push('\0');
        Self { s }
    }
}

impl AsRef<PadStr> for RightPadString {
    fn as_ref(&self) -> &PadStr {
        let s: &str = self.as_str();
        unsafe { std::mem::transmute(s) }
    }
}

pub struct LeftPadString {
    s: String,
}

impl LeftPadString {
    pub fn push(&mut self, ch: char) {
        self.s.push(ch);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.s.len() == 1 {
            None
        } else {
            self.s.pop()
        }
    }

    pub fn as_str(&self) -> &str {
        &self.s[1..]
    }
}

impl From<String> for LeftPadString {
    fn from(s: String) -> Self {
        let mut pad = String::new();
        pad.push('\0');
        pad.push_str(&s);
        Self { s: pad }
    }
}

impl AsRef<PadStr> for LeftPadString {
    fn as_ref(&self) -> &PadStr {
        let s: &str = self.as_str();
        unsafe { std::mem::transmute(s) }
    }
}

#[repr(transparent)]
pub struct PadStr {
    s: str,
}

impl PadStr {
    /// Joins two contiguous PadStrs that were produced as a result of calling the
    /// split_at method.
    pub fn join_contig<'a>(&'a self, next: &'a PadStr) -> Option<&'a PadStr> {
        if next.is_empty() {
            return Some(self);
        }
        let len = self.len();
        let contig = unsafe { self.as_ptr().add(len) };
        if contig == next.as_ptr() {
            Some(unsafe {
                std::mem::transmute(std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    self.as_ptr(),
                    len + next.len(),
                )))
            })
        } else {
            None
        }
    }

    pub fn split_at(&self, mid: usize) -> (&PadStr, &PadStr) {
        let (s1, s2) = self.s.split_at(mid);
        unsafe { (mem::transmute(s1), mem::transmute(s2)) }
    }
}

macro_rules! impl_index_types {
    ( $( $x:ty ),+ ) => {
        $(
            impl Index<$x> for PadStr {
                type Output = PadStr;

                fn index(&self, i: $x) -> &PadStr {
                    unsafe { mem::transmute(self.s.index(i)) }
                }
            }

            impl IndexMut<$x> for PadStr {
                fn index_mut(&mut self, i: $x) -> &mut PadStr {
                    unsafe { mem::transmute(self.s.index_mut(i)) }
                }
            }
        )+
    }
}

impl_index_types! {
    Range<usize>,
    RangeFrom<usize>,
    RangeInclusive<usize>,
    RangeTo<usize>,
    RangeToInclusive<usize>
}

impl Deref for PadStr {
    type Target = str;

    fn deref(&self) -> &str {
        &self.s
    }
}

impl DerefMut for PadStr {
    fn deref_mut(&mut self) -> &mut str {
        &mut self.s
    }
}
