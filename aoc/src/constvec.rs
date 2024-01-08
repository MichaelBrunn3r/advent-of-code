use std::fmt::{Debug, Formatter};

pub struct ConstVec<T, const C: usize> {
    pub data: [T; C],
    pub len: u16,
}

impl<T, const C: usize> ConstVec<T, C> {
    pub fn push(&mut self, value: T) {
        self.data[self.len as usize] = value;
        self.len += 1;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T: Debug, const C: usize> Debug for ConstVec<T, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.data[..self.len as usize].iter())
            .finish()
    }
}

impl<T, const C: usize> AsRef<[T]> for ConstVec<T, C> {
    fn as_ref(&self) -> &[T] {
        &self.data[..self.len as usize]
    }
}
