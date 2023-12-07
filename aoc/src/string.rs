use std::{fmt::Debug, str::FromStr};

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
}