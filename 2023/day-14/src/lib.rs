use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const ROUND_ROCK: u8 = b'O';
const CUBE_ROCK: u8 = b'#';
const EMPTY: u8 = b'.';

pub fn part_1(input: &str) -> usize {
    let num_cols = input.lines().next().unwrap().len();
    let num_rows = input.lines().count();

    let mut current_stops = vec![num_rows + 1; num_cols];
    let mut total_load = 0;

    for (line, load) in input
        .as_bytes()
        .chunks_exact(num_cols + 1)
        .zip((1..num_rows + 1).rev())
    {
        for (col, c) in line.iter().enumerate() {
            match *c {
                ROUND_ROCK => {
                    current_stops[col] -= 1;
                    total_load += current_stops[col];
                }
                CUBE_ROCK => {
                    current_stops[col] = load;
                }
                _ => {}
            }
        }
    }

    total_load
}

pub fn part_2(input: &str) -> usize {
    0
}
