use std::collections::HashMap;

use aoc::prelude::*;
use itertools::Itertools;

// 1622577954 too high
pub fn p1(input: &str) -> usize {
    let mut stones = input[..input.len()-1].split(" ").map(|stone| stone.as_bytes().parse_ascii_digits()).collect_vec();
    for _ in 0..25 {
        let mut new_stones = Vec::new();

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }
            
            let num_digits = stone.digits();
            if num_digits.even() {
                let left = stone / (10usize.pow(num_digits as u32 / 2));
                let right = stone % (10usize.pow(num_digits as u32 / 2));

                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024);
            }
        }

        stones = new_stones;
    }

    stones.len()
}

pub fn p2(input: &str) -> usize {
    0
}
