use std::ops::Range;

pub trait U8SliceExt {
    fn as_str_unchecked(&self) -> &str;
    fn split_at_range_unchecked(&self, range: &Range<usize>) -> (&[u8], &[u8]);
    fn parse_ascii_digits(&self) -> usize;
    fn parse_n_ascii_digits(&self, n: usize) -> u32;
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

    fn parse_ascii_digits(&self) -> usize {
        let mut num = (self[0] - b'0') as usize;

        for &c in &self[1..self.len()] {
            num *= 10;
            num += (c - b'0') as usize;
        }
        num
    }

    fn parse_n_ascii_digits(&self, n: usize) -> u32 {
        debug_assert!(n >= 1);
        debug_assert!(n <= self.len());

        let mut num = (self[0] - b'0') as u32;

        for &c in &self[1..n] {
            num *= 10;
            num += (c - b'0') as u32;
        }

        num
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
