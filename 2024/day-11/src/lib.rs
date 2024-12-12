use aoc::prelude::*;
use dashmap::DashMap;
use lazy_static::lazy_static;
use rayon::{iter::ParallelIterator, str::ParallelString};

lazy_static! {
    static ref MEMO: DashMap<(u8, usize), usize> = DashMap::new();
}

pub fn p(input: &str) -> (usize, usize) {
    input[..input.len() - 1]
        .par_split(' ')
        .map(|stone| stone.as_bytes().parse_ascii_digits())
        .map(|stone| (blink(25, stone), blink(75, stone)))
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn blink(n: u8, stone: usize) -> usize {
    if let Some(cached) = MEMO.get(&(n, stone)) {
        return *cached;
    }

    let num_stones = if n == 0 {
        1
    } else if stone == 0 {
        blink(n - 1, 1)
    } else {
        let num_digits = stone.digits();
        if num_digits.even() {
            let split = 10usize.pow(num_digits as u32 / 2);
            blink(n - 1, stone / split) + blink(n - 1, stone % split)
        } else {
            blink(n - 1, stone * 2024)
        }
    };

    MEMO.insert((n, stone), num_stones);
    num_stones
}
