use std::ops::Range;

pub trait RangeExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn without_unchecked(&self, other: &Self) -> (Range<T>, Range<T>);
}

impl RangeExt<usize> for Range<usize> {
    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn without_unchecked(&self, other: &Self) -> (Range<usize>, Range<usize>) {
        let left = self.start..other.start;
        let right = other.end..self.end;

        (left, right)
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
