use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    input[..input.len() - 1]
        .split(',')
        .map(|step| {
            step.as_bytes()
                .into_iter()
                .fold(0u8, |acc, &c| (acc.wrapping_add(c)).wrapping_mul(17)) as usize
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}
