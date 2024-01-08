use crate::prelude::*;
use std::ops::{AddAssign, MulAssign};

pub trait U8PtrExt {
    fn as_str(&self, n: usize) -> &str;

    fn parse_ascii_digits(&mut self, num_digits: usize) -> usize;
    fn parse_uint<T: From<u8> + MulAssign + AddAssign, const N: usize>(&mut self) -> T;
    fn parse_n_uints<T: From<u8> + MulAssign + AddAssign, const N: usize, const D: usize>(
        &mut self,
        seperator: usize,
    ) -> [T; N];
    fn parse_int<T: From<u8> + From<i32> + MulAssign + AddAssign, const DIGITS: usize>(
        &mut self,
    ) -> T;
    fn parse_n_ints<
        T: From<u8> + From<i32> + MulAssign + AddAssign,
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

    fn parse_ascii_digits(&mut self, num_digits: usize) -> usize {
        unsafe {
            let mut num = (self.read() - b'0') as usize;
            *self = self.add(1);

            for _ in 1..num_digits {
                num *= 10;
                num += (self.read() - b'0') as usize;
                *self = self.add(1);
            }
            num
        }
    }

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

    fn parse_int<T: From<u8> + From<i32> + MulAssign + AddAssign, const DIGITS: usize>(
        &mut self,
    ) -> T {
        unsafe {
            let sign = if self.read() == b'-' {
                *self = self.add(1);
                -1
            } else {
                1
            };

            let mut num: T = self.parse_uint::<T, DIGITS>();
            num *= sign.into();
            num
        }
    }

    fn parse_n_ints<
        T: From<u8> + From<i32> + MulAssign + AddAssign,
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
