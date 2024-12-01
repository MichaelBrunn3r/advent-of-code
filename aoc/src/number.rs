use gcd::Gcd;
use num_traits::Unsigned;

pub trait UnsignedExt: Unsigned {
    fn lcm(self, other: Self) -> Self;
    /** https://en.wikipedia.org/wiki/Hamming_distance */
    fn hamming_distance(&self, other: &Self) -> usize;
}

impl UnsignedExt for usize {
    fn lcm(self, other: Self) -> Self {
        self * other / self.gcd(other)
    }

    fn hamming_distance(&self, other: &Self) -> usize {
        (*self ^ *other).count_ones() as usize
    }
}
