use std::ops::{self, Range, RangeInclusive};

pub trait RangeExt<T>
where
    Self: Sized,
{
    fn intersects(&self, other: &Self) -> bool;
    fn without_unchecked(&self, other: &Self) -> (Self, Self);
    fn intersection(&self, other: &Self) -> Self;
}

impl<T> RangeExt<T> for Range<T>
where
    T: PartialOrd + Copy + Ord,
{
    fn intersects(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn without_unchecked(&self, other: &Self) -> (Range<T>, Range<T>) {
        let left = self.start..other.start;
        let right = other.end..self.end;

        (left, right)
    }

    fn intersection(&self, other: &Self) -> Self {
        self.start.min(other.start)..self.end.max(other.end)
    }
}

impl<T> RangeExt<T> for RangeInclusive<T>
where
    T: PartialOrd + Copy + Ord,
{
    fn intersects(&self, other: &Self) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }

    fn without_unchecked(&self, other: &Self) -> (RangeInclusive<T>, RangeInclusive<T>) {
        let left = *self.start()..=*other.start();
        let right = *other.end()..=*self.end();

        (left, right)
    }

    fn intersection(&self, other: &Self) -> Self {
        *self.start().max(other.start())..=*self.end().min(other.end())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlaps() {
        assert!((0..5).intersects(&(0..5)));
        assert!((0..5).intersects(&(0..10)));
        assert!((0..5).intersects(&(2..10)));
        assert!((0..5).intersects(&(4..10)));

        assert!(!(0..5).intersects(&(5..10)));
    }
}
