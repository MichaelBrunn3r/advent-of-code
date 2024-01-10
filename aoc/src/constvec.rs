use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

pub struct ConstVec<T, const CAPACITY: usize> {
    pub data: [T; CAPACITY],
    pub len: u16,
}

impl<T: Copy, const CAPACITY: usize> ConstVec<T, CAPACITY> {
    #[allow(clippy::uninit_assumed_init)]
    pub const fn new(fill: T) -> Self {
        Self {
            data: [fill; CAPACITY],
            len: 0,
        }
    }

    pub const fn with_length(fill: T, len: u16) -> Self {
        Self {
            data: [fill; CAPACITY],
            len,
        }
    }

    #[track_caller]
    pub fn push(&mut self, value: T) {
        self.data[self.len as usize] = value;
        self.len += 1;
    }

    #[track_caller]
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

impl<T, const CAPACITY: usize> Deref for ConstVec<T, CAPACITY> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}

impl<T, const CAPACITY: usize> DerefMut for ConstVec<T, CAPACITY> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut()
    }
}

impl<T, const CAPACITY: usize> std::ops::Index<usize> for ConstVec<T, CAPACITY> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
