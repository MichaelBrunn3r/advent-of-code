use rand::seq::SliceRandom;
use std::{fmt::Debug, ops, str::FromStr};
pub trait CharExt {
    fn repeat(self, n: usize) -> String;
}

impl CharExt for char {
    fn repeat(self, n: usize) -> String {
        std::iter::repeat(self).take(n).collect()
    }
}

pub trait StrExt {
    fn parse_splits<'a, T: FromStr>(
        &'a self,
        delimiter: &'a str,
    ) -> Box<dyn Iterator<Item = T> + '_>
    where
        <T as FromStr>::Err: Debug;

    fn take_random(&self, n: usize) -> String;

    /** Parses a string into a u32 without checking for invalid characters or overflows */
    fn parse_unsigned_unchecked<U>(&self) -> U
    where
        U: Default + From<u8> + std::ops::Mul<Output = U> + ops::Add<Output = U>;
    fn repeat(&self, n: usize) -> String;
}

impl StrExt for str {
    fn parse_splits<'a, T: FromStr>(
        &'a self,
        delimiter: &'a str,
    ) -> Box<dyn Iterator<Item = T> + '_>
    where
        <T as FromStr>::Err: Debug,
    {
        Box::new(self.split(delimiter).map(|s| s.parse::<T>().unwrap()))
    }

    fn take_random(&self, n: usize) -> String {
        let mut chars = self.chars().collect::<Vec<char>>();
        chars.shuffle(&mut rand::thread_rng());
        chars.iter().take(n).collect::<String>()
    }

    fn parse_unsigned_unchecked<U>(&self) -> U
    where
        U: Default + From<u8> + ops::Mul<Output = U> + ops::Add<Output = U>,
    {
        let mut val = U::default();

        for c in self.bytes() {
            val = val * 10.into() + U::from(c - b'0');
        }

        val
    }

    fn repeat(&self, n: usize) -> String {
        std::iter::repeat(self).take(n).collect()
    }
}
