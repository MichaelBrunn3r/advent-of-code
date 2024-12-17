use crate::prelude::*;
use std::ops::{AddAssign, MulAssign};

pub trait U8PtrExt {
    fn as_str(&self, n: usize) -> &str;
    fn skip(&mut self, n: usize) -> &mut Self;
    fn take(&mut self) -> u8;
    fn peek(&self) -> u8;
    fn find(&self, needle: u8) -> usize;

    // unsigned int
    fn parse_uint<T: From<u8> + MulAssign + AddAssign, const N: usize>(&mut self) -> T;
    fn parse_uint_n_digits<T: From<u8> + MulAssign + AddAssign>(&mut self, digits: usize) -> T;
    fn parse_n_uints<T: From<u8> + MulAssign + AddAssign, const N: usize, const D: usize>(
        &mut self,
        seperator: usize,
    ) -> [T; N];

    // signed int
    fn parse_int<T: From<u8> + From<i8> + MulAssign + AddAssign, const DIGITS: usize>(
        &mut self,
    ) -> T;
    fn parse_n_ints<
        T: From<u8> + From<i8> + MulAssign + AddAssign,
        const N: usize,
        const D: usize,
    >(
        &mut self,
        seperator: usize,
    ) -> [T; N];
}

impl U8PtrExt for *const u8 {
    fn as_str(&self, n: usize) -> &str {
        unsafe { std::slice::from_raw_parts(*self, n).as_str_unchecked() }
    }
    fn skip(&mut self, n: usize) -> &mut Self {
        *self = unsafe { self.add(n) };
        self
    }
    fn take(&mut self) -> u8 {
        unsafe {
            let val = self.read();
            *self = self.add(1);
            val
        }
    }
    fn peek(&self) -> u8 {
        unsafe { self.read() }
    }
    fn find(&self, needle: u8) -> usize {
        let mut i = 0;
        loop {
            if unsafe{*self.add(i)} == needle {
                return i;
            }
            i += 1;
        }
    }

    #[track_caller]
    fn parse_uint<T: From<u8> + MulAssign + AddAssign, const DIGITS: usize>(&mut self) -> T {
        unsafe {
            let mut num: T = 0.into();
            for _ in 0..DIGITS {
                num *= 10.into();
                num += (self.read() - b'0').into();
                *self = self.add(1);
            }
            while self.read() >= b'0' {
                num *= 10.into();
                num += (self.read() - b'0').into();
                *self = self.add(1);
            }
            num
        }
    }

    #[track_caller]
    fn parse_uint_n_digits<T: From<u8> + MulAssign + AddAssign>(&mut self, digits: usize) -> T {
        unsafe {
            let mut num: T = (self.read() - b'0').into();
            *self = self.add(1);

            for _ in 1..digits {
                num *= 10.into();
                num += (self.read() - b'0').into();
                *self = self.add(1);
            }
            num
        }
    }

    #[track_caller]
    fn parse_n_uints<T: From<u8> + MulAssign + AddAssign, const N: usize, const DIGITS: usize>(
        &mut self,
        seperator: usize,
    ) -> [T; N] {
        unsafe {
            let mut nums: [T; N] = std::mem::zeroed();
            for num in &mut nums {
                *num = self.parse_uint::<T, DIGITS>();
                *self = self.add(seperator);
            }
            *self = self.sub(seperator);
            nums
        }
    }

    #[track_caller]
    fn parse_int<T: From<u8> + From<i8> + MulAssign + AddAssign, const DIGITS: usize>(
        &mut self,
    ) -> T {
        unsafe {
            let sign = if self.read() == b'-' {
                *self = self.add(1);
                -1i8
            } else {
                1
            };

            let mut num: T = self.parse_uint::<T, DIGITS>();
            num *= sign.into();
            num
        }
    }

    #[track_caller]
    fn parse_n_ints<
        T: From<u8> + From<i8> + MulAssign + AddAssign,
        const N: usize,
        const DIGITS: usize,
    >(
        &mut self,
        seperator: usize,
    ) -> [T; N] {
        unsafe {
            let mut nums: [T; N] = std::mem::zeroed();
            for num in &mut nums {
                *num = self.parse_int::<T, DIGITS>();
                *self = self.add(seperator);
            }
            *self = self.sub(seperator);
            nums
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cursor<T> {
    pub ptr: *const T,
}

impl<T> Cursor<T> {
    pub fn skip(&mut self, n: usize) {
        unsafe { self.ptr = self.ptr.add(n) }
    }

    pub fn take(&mut self) -> T {
        unsafe {
            let val = self.ptr.read();
            self.ptr = self.ptr.add(1);
            val
        }
    }
}

impl Cursor<u8> {
    #[track_caller]
    pub fn parse_uint_n_digits<T: From<u8> + MulAssign + AddAssign>(&mut self, digits: usize) -> T {
        unsafe {
            let mut num: T = (self.ptr.read() - b'0').into();
            self.ptr = self.ptr.add(1);

            for _ in 1..digits {
                num *= 10.into();
                num += (self.ptr.read() - b'0').into();
                self.ptr = self.ptr.add(1);
            }
            num
        }
    }
}

impl<T> From<*const T> for Cursor<T> {
    fn from(ptr: *const T) -> Self {
        Self { ptr }
    }
}

impl<T> std::ops::Index<usize> for Cursor<T> {
    type Output = T;
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.ptr.add(index) }
    }
}
