use std::ops::Range;

pub trait RangeExt {
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeExt for Range<usize> {
    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlaps() {
        assert!((0..5).overlaps(&(0..5)));
        assert!((0..5).overlaps(&(0..10)));
        assert!((0..5).overlaps(&(2..10)));
        assert!((0..5).overlaps(&(4..10)));

        assert!(!(0..5).overlaps(&(5..10)));
    }
}
