pub trait SliceExt<T> {
    fn partialy_reflects_at(&self, idx: usize) -> bool;
}

impl<T: PartialEq> SliceExt<T> for [T] {
    fn partialy_reflects_at(&self, idx: usize) -> bool {
        let dist = (self.len() - idx).min(idx);

        for i in 0..dist {
            if self[idx - i - 1] != self[idx + i] {
                return false;
            }
        }

        return true;
    }
}

pub trait NumericSliceExt<T> {
    fn sum(&self) -> T;
    fn mean(&self) -> T;
}

impl<T> NumericSliceExt<T> for [T]
where
    T: for<'a> std::iter::Sum<&'a T> + std::ops::Div<usize, Output = T> + std::ops::Sub,
    
{
    fn sum(&self) -> T {
        self.iter().sum()
    }

    fn mean(&self) -> T {
        self.sum() / self.len()
    }
}
