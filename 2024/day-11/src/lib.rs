use aoc::prelude::*;
use fxhash::FxHashMap;
use memoize::memoize;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn p(input: &str) -> (usize, usize) {
    input[..input.len() - 1]
        .par_split(' ')
        .map(|stone| stone.as_bytes().parse_ascii_digits())
        .map(|stone| (blink(25, stone), blink(75, stone)))
        .reduce(|| (0,0), |a,b| (a.0+b.0,a.1+b.1))
}

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default(), SharedCache)]
fn blink(n: u8, stone: usize) -> usize {
    return if n == 0 {
        1
    } else if stone == 0 {
        blink(n-1, 1)
    } else {
        let num_digits = stone.digits();
        if num_digits.even() {
            let split = 10usize.pow(num_digits as u32 / 2);
            blink(n-1, stone / split) + blink(n-1, stone % split)
        } else {
            blink(n-1, stone*2024)
        }
    }
}
