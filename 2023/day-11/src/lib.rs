use std::collections::HashSet;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: String) -> usize {
    let size = input.find('\n').unwrap();

    let mut galaxies = vec![vec![]; size];
    let mut col_is_empty = vec![true; size];

    let mut vertical_expansion = 0;
    for (row, line) in input.lines().enumerate() {
        let mut is_row_empty = true;
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                is_row_empty = false;
                col_is_empty[col] = false;
                galaxies[col].push(Galaxy(row + vertical_expansion, col));
            }
        }

        if is_row_empty {
            vertical_expansion += 1;
        }
    }

    let mut horizontal_expansion = 0;
    for (col, is_empty) in col_is_empty.into_iter().enumerate() {
        if is_empty {
            horizontal_expansion += 1;
        } else {
            for galaxy in &mut galaxies[col] {
                galaxy.1 += horizontal_expansion;
            }
        }
    }

    galaxies
        .into_iter()
        .flatten()
        .tuple_combinations()
        .map(|(a, b)| {
            let vert_dist = (a.0 as isize - b.0 as isize).abs();
            let horiz_dist = (a.1 as isize - b.1 as isize).abs();
            (vert_dist + horiz_dist) as usize
        })
        .sum()
}

pub fn part_2(input: String) -> usize {
    0
}

#[derive(Debug, Clone, Copy)]
struct Galaxy(usize, usize);
