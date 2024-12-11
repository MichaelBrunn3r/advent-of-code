use std::collections::HashMap;
use aoc::prelude::*;
use itertools::Itertools;
use fxhash::FxHashMap;

pub fn p1(input: &str) -> usize {
    let stones = input[..input.len()-1].split(" ").map(|stone| stone.as_bytes().parse_ascii_digits()).collect_vec();
    let mut cache = FxHashMap::default();

    let mut num_stones = 0;
    for stone in stones {
        num_stones += blink(25, stone, &mut cache);
    }

    num_stones
}

pub fn p2(input: &str) -> usize {
    let stones = input[..input.len()-1].split(" ").map(|stone| stone.as_bytes().parse_ascii_digits()).collect_vec();
    let mut cache = FxHashMap::default();

    let mut num_stones = 0;
    for stone in stones {
        num_stones += blink(75, stone, &mut cache);
    }

    num_stones
}

fn blink(n: u8, stone: usize, cache: &mut FxHashMap<(u8, usize), usize>) -> usize {
    let num_stones = if n == 0 {
        1
    } else if let Some(cached) = cache.get(&(n, stone)) {
        *cached
    } else if stone == 0 {
        blink(n-1, 1, cache)
    } else {
        let num_digits = stone.digits();
        if num_digits.even() {
            let left = stone / (10usize.pow(num_digits as u32 / 2));
            let right = stone % (10usize.pow(num_digits as u32 / 2));

            blink(n-1, left, cache) + blink(n-1, right, cache)
        } else {
            blink(n-1, stone*2024, cache)
        }
    };

    cache.insert((n, stone), num_stones);
    num_stones
}
