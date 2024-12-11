use std::collections::HashMap;

use aoc::prelude::*;
use itertools::Itertools;

pub fn p(input: &str) -> (usize, usize) {
    let mut stones = input[..input.len() - 1]
        .split(" ")
        .map(|stone| stone.as_bytes().parse_ascii_digits())
        .counts();

    let stones_25 = count_stones(25, &mut stones);
    let stones_75 = count_stones(50, &mut stones);

    (stones_25, stones_75)
}

fn count_stones(blinks: u8, stones: &mut HashMap<usize, usize>) -> usize {
    let mut new = HashMap::new(); // see Note 1

    for _ in 0..blinks {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new.entry(1).or_default() += count;
            } else {
                let num_digits = stone.digits();
                if num_digits.even() {
                    let left = stone / (10usize.pow(num_digits as u32 / 2));
                    let right = stone % (10usize.pow(num_digits as u32 / 2));
        
                    *new.entry(left).or_default() += count;
                    *new.entry(right).or_default() += count;
                } else {
                    *new.entry(stone * 2024).or_default() += count;
                }
            }
        }

        std::mem::swap(stones, &mut new);
    }

    stones.values().sum()
}

// Note 1: The trick is that we deduplicate all stones.
//         In each iteration we process e.g. ALL stones with a 1, ALL stones with a 23425, ...

