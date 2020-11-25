use std::mem;
use std::ops::{
    Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive,
};

pub struct PadString {
    s: String,
}

impl From<String> for PadString {
    fn from(mut s: String) -> Self {
        s.push('\0');
        Self { s }
    }
}

impl Into<String> for PadString {
    fn into(mut self) -> String {
        let _ = self.s.pop();
        self.s
    }
}

impl PadString {
    pub fn as_str(&self) -> &str {
        let total_len = self.s.len();
        &self.s[0..total_len - 1]
    }
}

impl AsRef<PadStr> for PadString {
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
