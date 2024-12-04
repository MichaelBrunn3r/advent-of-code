#![feature(stdsimd)]

use aoc::{prelude::*, ConstVec};
use std::{arch::x86_64::_mm_prefetch, collections::HashMap};

const SPHERE: u8 = b'O'; // 0100_1111;
const CUBE: u8 = b'#'; //   0010_0010;
const EMPTY: u8 = b'.'; //  0010_1110;
const COLS: usize = 101;

pub fn p1(input: &str) -> usize {
    tilt_north_and_calc_load(input)
}

pub fn p2(input: &mut str) -> usize {
    let size = input.find('\n').unwrap();
    let platform = unsafe { input.as_bytes_mut() };

    let (spins, cycle_period) = spin_until_first_cycle_period(platform, size);

    let spins_left = (1_000_000_000 - spins) % cycle_period;
    for _ in 0..spins_left {
        spin(platform, size, size + 1);
    }

    calc_load(platform, size)
}

// Part 1 doesn't need any preprocessing, so we can just use the input string as is.
pub fn tilt_north_and_calc_load(input: &str) -> usize {
    let mut column_stops = [0; 100];
    let mut total_load = 0;

    let mut crs = input.as_ptr();
    for row in 0..100 {
        unsafe {
            _mm_prefetch(crs.add(64) as *const _, 0);
            _mm_prefetch(crs.add(128) as *const _, 0);
            _mm_prefetch(column_stops.as_ptr() as *const _, 0);
            _mm_prefetch(column_stops.as_ptr().add(64) as *const _, 0);
        };
        for stop in &mut column_stops {
            match crs.take() {
                SPHERE => {
                    total_load += 100 - *stop;
                    *stop += 1;
                }
                CUBE => {
                    *stop = row + 1;
                }
                _ => {}
            }
        }
        crs.skip("\n".len());
    }

    total_load
}

pub fn calc_load(platform: &[u8], size: usize) -> usize {
    let mut total_load = 0;

    for (line, load) in platform.chunks_exact(size + 1).zip((1..size + 1).rev()) {
        for c in line {
            if *c == SPHERE {
                total_load += load;
            }
        }
    }

    total_load
}

fn spin_until_first_cycle_period(platform: &mut [u8], size: usize) -> (usize, usize) {
    let mut after_spin_states = HashMap::with_capacity(128);
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
    let mut row_stops = [0; COLS];

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
    let mut row_stops = [rows - 1; COLS];

    let mut pos = rows * cols - 1;
    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            let c = platform[pos];
            match c {
                SPHERE => {
                    platform[pos] = EMPTY;
                    platform[row_stops[col] * cols + col] = SPHERE;
                    row_stops[col] = row_stops[col].wrapping_sub(1);
                }
                CUBE => {
                    row_stops[col] = row.wrapping_sub(1);
                }
                _ => {}
            }
            pos = pos.wrapping_sub(1);
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
                    stop = stop.wrapping_sub(1);
                }
                CUBE => {
                    stop = col;
                }
                _ => {}
            }
            pos = pos.wrapping_sub(1);
        }
        row_start = row_start.wrapping_sub(cols);
    }
}
