use std::{
    fmt::{Debug, Formatter},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr,
};

pub struct ConstVec<D, const CAPACITY: usize> {
    pub data: [MaybeUninit<D>; CAPACITY],
    pub len: u16,
}

impl<D, const CAPACITY: usize> ConstVec<D, CAPACITY> {
    const EMPTY: MaybeUninit<D> = MaybeUninit::uninit();

    pub const fn new() -> Self {
        Self {
            data: [Self::EMPTY; CAPACITY],
            len: 0,
        }
    }

    #[allow(clippy::uninit_assumed_init)]
    pub const fn new_filled(fill: D) -> Self
    where
        D: Copy,
    {
        let mut data: [MaybeUninit<D>; CAPACITY] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        while i < CAPACITY {
            data[i] = MaybeUninit::new(fill);
            i += 1;
        }
        Self {
            data,
            len: CAPACITY as u16,
        }
    }

    pub const fn zeroed() -> Self {
        Self {
            data: unsafe { std::mem::zeroed() },
            len: 0,
        }
    }

    #[track_caller]
    pub fn push(&mut self, value: D) {
        debug_assert!(self.len < CAPACITY as u16);
        unsafe { ptr::write(self.as_mut_ptr().add(self.len as usize), value) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<D> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            self.len -= 1;
            Some(std::ptr::read(self.as_ptr().add(self.len as usize)))
        }
    }

    #[track_caller]
    pub fn emplace_back(&mut self) -> &mut D {
        let elem = unsafe { &mut *self.as_mut_ptr().add(self.len as usize) };
        self.len += 1;
        elem
    }

    #[track_caller]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, D> {
        self.as_ref().iter()
    }

    fn as_ptr(&self) -> *const D {
        self.data.as_ptr() as *const D
    }

    fn as_mut_ptr(&mut self) -> *mut D {
        self.data.as_mut_ptr() as *mut D
    }
}

impl<D: Debug, const C: usize> Debug for ConstVec<D, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<D, const C: usize> AsRef<[D]> for ConstVec<D, C> {
    fn as_ref(&self) -> &[D] {
        unsafe { &*(&self.data[..self.len as usize] as *const [MaybeUninit<D>] as *const [D]) }
    }
}

impl<D, const CAPACITY: usize> Deref for ConstVec<D, CAPACITY> {
    type Target = [D];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<D, const CAPACITY: usize> DerefMut for ConstVec<D, CAPACITY> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut self.data[..self.len as usize] as *mut [MaybeUninit<D>] as *mut [D]) }
    }
}

impl<D, const CAPACITY: usize> std::ops::Index<usize> for ConstVec<D, CAPACITY> {
    type Output = D;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl<D, const CAP: usize> Drop for ConstVec<D, CAP> {
    fn drop(&mut self) {
        self.clear();

        // MaybeUninit inhibits array's drop
    }
}
