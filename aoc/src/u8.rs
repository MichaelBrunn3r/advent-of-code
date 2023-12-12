use std::ops::Range;

pub trait U8SliceExt {
    fn as_str_unchecked(&self) -> &str;
    fn split_at_range_unchecked(&self, range: &Range<usize>) -> (&[u8], &[u8]);
}

impl U8SliceExt for [u8] {
    fn as_str_unchecked(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self) }
    }

    fn split_at_range_unchecked(&self, range: &Range<usize>) -> (&[u8], &[u8]) {
        let left = &self[0..range.start];
        let right = &self[range.end..self.len()];
        (left, right)
    }
}

pub trait SliceOfU8SlicesExt {
    fn as_strs_unchecked(&self) -> Vec<&str>;
}

impl SliceOfU8SlicesExt for [&[u8]] {
    fn as_strs_unchecked(&self) -> Vec<&str> {
        self.iter().map(|s| s.as_str_unchecked()).collect()
    }
}
