use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const SPHERE: u8 = b'O';
const CUBE: u8 = b'#';
const EMPTY: u8 = b'.';

pub fn part_1(input: &str) -> usize {
    tilt_north_and_calc_load(input.as_bytes(), input.find('\n').unwrap())
}

// 56317 too low
// 96325 too high
pub fn part_2(input: &mut str) -> usize {
    let size = input.find('\n').unwrap();
    let platform = unsafe { input.as_bytes_mut() };

    let (spins, cycle_len) = spin_until_repeating(platform, size);

    let spins_left = (1_000_000_000 - spins) % cycle_len;
    for _ in 0..spins_left {
        spin(platform, size, size + 1);
    }

    calc_load(platform, size)
}

pub fn calc_load(platform: &[u8], size: usize) -> usize {
    let mut total_load = 0;

    for (line, load) in platform.chunks_exact(size + 1).zip((1..size + 1).rev()) {
        for c in line {
            match *c {
                SPHERE => {
                    total_load += load;
                }
                _ => {}
            }
        }
    }

    total_load
}

pub fn tilt_north_and_calc_load(platform: &[u8], size: usize) -> usize {
    let mut current_stops = vec![size + 1; size];
    let mut total_load = 0;

    for (line, load) in platform.chunks_exact(size + 1).zip((1..size + 1).rev()) {
        for (col, c) in line.iter().enumerate() {
            match *c {
                SPHERE => {
                    current_stops[col] -= 1;
                    total_load += current_stops[col];
                }
                CUBE => {
                    current_stops[col] = load;
                }
                _ => {}
            }
        }
    }

    total_load
}

fn spin_until_repeating(platform: &mut [u8], size: usize) -> (usize, usize) {
    let mut after_spin_states = HashMap::new();
    let mut i = 0;
    loop {
        i += 1;
        spin(platform, size, size + 1);
        if let Some(duplicate) = after_spin_states.insert(platform.as_str_unchecked().to_owned(), i)
        {
            return (i, i - duplicate);
        };
    }
}

pub fn spin(platform: &mut [u8], rows: usize, cols: usize) {
    tilt_north(platform, rows, cols);
    tilt_west(platform, rows, cols);
    tilt_south(platform, rows, cols);
    tilt_east(platform, rows, cols);
}

pub fn tilt_north(platform: &mut [u8], rows: usize, cols: usize) {
    let mut row_stops = vec![0; cols];

    let mut pos = 0;
    for row in 0..rows {
        for col in 0..cols {
            let c = platform[pos];
            match c {
                SPHERE => {
                    platform[pos] = EMPTY;
                    platform[row_stops[col] * cols + col] = SPHERE;
                    row_stops[col] += 1;
                }
                CUBE => {
                    row_stops[col] = row + 1;
                }
                _ => {}
            }
            pos += 1;
        }
    }
}

fn tilt_south(platform: &mut [u8], rows: usize, cols: usize) {
    let mut row_stops = vec![rows - 1; cols];

    let mut pos = rows * cols - 1;
    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            let c = platform[pos];
            match c {
                SPHERE => {
                    platform[pos] = EMPTY;
                    platform[row_stops[col] * cols + col] = SPHERE;
                    row_stops[col] = row_stops[col].saturating_sub(1);
                }
                CUBE => {
                    row_stops[col] = row.saturating_sub(1);
                }
                _ => {}
            }
            pos = pos.saturating_sub(1);
        }
    }
}

fn tilt_west(platform: &mut [u8], rows: usize, cols: usize) {
    let mut pos = 0;
    let mut row_start = 0;
    for _ in 0..rows {
        let mut stop = 0;
        for col in 0..cols {
            let c = platform[pos];
            match c {
                SPHERE => {
                    platform[pos] = EMPTY;
                    platform[row_start + stop] = SPHERE;
                    stop += 1;
                }
                CUBE => {
                    stop = col + 1;
                }
                _ => {}
            }
            pos += 1;
        }
        row_start += cols;
    }
}

fn tilt_east(platform: &mut [u8], rows: usize, cols: usize) {
    let mut pos = rows * cols - 1;
    let mut row_start = rows * cols - cols;
    for _ in (0..rows).rev() {
        let mut stop = cols - 1;
        for col in (0..cols).rev() {
            let c = platform[pos];
            match c {
                SPHERE => {
                    platform[pos] = EMPTY;
                    platform[row_start + stop - 1] = SPHERE;
                    stop = stop.saturating_sub(1);
                }
                CUBE => {
                    stop = col;
                }
                _ => {}
            }
            pos = pos.saturating_sub(1);
        }
        row_start = row_start.saturating_sub(cols);
    }
}
