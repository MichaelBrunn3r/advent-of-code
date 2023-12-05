pub trait CharExt {
    fn repeat(self, n: usize) -> String;
}

impl CharExt for char {
    fn repeat(self, n: usize) -> String {
        std::iter::repeat(self).take(n).collect()
    }
}
