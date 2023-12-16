use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const EMPTY: u8 = b'.';
const MIRROR_R: u8 = b'/';
const MIRROR_L: u8 = b'\\';
const SPLITTER_V: u8 = b'|';
const SPLITTER_H: u8 = b'-';

// Calc vert dir changes: 12.483 µs
pub fn part_1(input: &str) -> usize {
    let width = input.find('\n').unwrap();

    let vert_dir_changes = input
        .as_bytes()
        .chunks_exact(width + 1)
        .map(|line| {
            let mut dir_change = 0u128;
            for (i, &c) in line.iter().enumerate() {
                let is_dir_change = c == MIRROR_L || c == MIRROR_R || c == SPLITTER_V;
                dir_change |= (is_dir_change as u128) << i;
            }
            dir_change
        })
        .collect_vec();

    // vert_dir_changes
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, dir_change)| {
    //         println!(
    //             "{}",
    //             input
    //                 .lines()
    //                 .nth(i)
    //                 .unwrap()
    //                 .chars()
    //                 .rev()
    //                 .collect::<String>()
    //         );
    //         println!("{:0110b}", dir_change);
    //     });

    vert_dir_changes
        .iter()
        .map(|&row| row.count_ones() as usize)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}
