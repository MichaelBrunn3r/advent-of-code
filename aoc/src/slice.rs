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
