use gcd::Gcd;
use num_traits::Unsigned;

pub trait UnsignedExt: Unsigned {
    fn lcm(self, other: Self) -> Self;
    /** https://en.wikipedia.org/wiki/Hamming_distance */
    fn hamming_distance(&self, other: &Self) -> usize;
    fn even(&self) -> bool;
    fn digits(self) -> usize;
    fn bits(&self) -> usize;
    fn coprime(&self, other: Self) -> bool;
}

impl UnsignedExt for usize {
    fn lcm(self, other: Self) -> Self {
        self * other / self.gcd(other)
    }

    fn hamming_distance(&self, other: &Self) -> usize {
        (*self ^ *other).count_ones() as usize
    }

    fn even(&self) -> bool {
        self & 1 == 0
    }

    fn digits(self) -> usize {
        static POW_OF_10: [usize; 21] = [
            0,
            1,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            1000000000,
            10000000000,
            100000000000,
            1000000000000,
            10000000000000,
            100000000000000,
            1000000000000000,
            10000000000000000,
            100000000000000000,
            1000000000000000000,
            10000000000000000000,
        ];
        static MAX_DIGITS: [u8; 65] = [
            1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9,
            9, 10, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16,
            16, 16, 16, 17, 17, 17, 18, 18, 18, 19, 19, 19, 19, 20,
        ];

        if self == 0 {
            return 1;
        }

        // let bits = size_of::<usize>() * 8 - self.leading_zeros() as usize;
        let mut digits = MAX_DIGITS[self.bits()] as usize;

        if self < POW_OF_10[digits] {
            digits -= 1;
        }

        digits
    }

    fn bits(&self) -> usize {
        size_of::<Self>() * 8 - self.leading_zeros() as usize
    }

    fn coprime(&self, other: Self) -> bool {
        self.gcd(other) == 1
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::UnsignedExt;

    #[test]
    fn num_digits() {
        let mut rng = rand::thread_rng();

        assert_eq!(0usize.digits(), 1);
        assert_eq!(usize::MAX.digits(), 20);

        for _ in 0..100_000 {
            let num = rng.gen_range(1..usize::MAX);
            assert_eq!(num.digits(), (num.ilog10() as usize) + 1);
        }
    }
}
