use std::ops::RangeInclusive;

use crate::prelude::CharExt;

pub trait RangeExt {
    fn from_tuple(tuple: (usize, usize)) -> Self;
    fn from_start_length(start: usize, length: usize) -> Self;
    fn len(&self) -> usize;
}

impl RangeExt for RangeInclusive<usize> {
    fn from_tuple(tuple: (usize, usize)) -> Self {
        tuple.0..=tuple.1
    }

    fn from_start_length(start: usize, length: usize) -> Self {
        start..=start + length
    }

    fn len(&self) -> usize {
        self.end() - self.start() + 1
    }
}
