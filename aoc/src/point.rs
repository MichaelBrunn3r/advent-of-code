pub trait PointExt {
    fn manhattan_distance(&self, rhs: (usize, usize)) -> usize;
}

impl PointExt for (usize, usize) {
    fn manhattan_distance(&self, rhs: (usize, usize)) -> usize {
        self.0.abs_diff(rhs.0) + self.1.abs_diff(rhs.1)
    }
}
