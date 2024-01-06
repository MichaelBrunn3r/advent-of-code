use std::collections::HashSet;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    sum_distances(input, 1)
}

pub fn part_2(input: &str, expansion_rate: usize) -> usize {
    sum_distances(input, expansion_rate)
}

const LINE_LEN: usize = 141;

fn sum_distances(input: &str, expansion_rate: usize) -> usize {
    let size = LINE_LEN;

    let mut galaxies = vec![vec![]; size];
    let mut col_is_empty = vec![true; size];

    let mut vertical_expansion = 0;
    for (row, line) in input.as_bytes().chunks_exact(LINE_LEN).enumerate() {
        let mut is_row_empty = true;
        for (col, &c) in line.iter().enumerate() {
            if c == b'#' {
                is_row_empty = false;
                col_is_empty[col] = false;
                galaxies[col].push((row + vertical_expansion, col));
            }
        }

        if is_row_empty {
            vertical_expansion += expansion_rate;
        }
    }

    let mut horizontal_expansion = 0;
    for (col, is_empty) in col_is_empty.into_iter().enumerate() {
        if is_empty {
            horizontal_expansion += expansion_rate;
        } else {
            for galaxy in &mut galaxies[col] {
                galaxy.1 += horizontal_expansion;
            }
        }
    }

    let mut sum = 0;
    for row in 0..galaxies.len() {
        for col in 0..galaxies[row].len() {
            for other_col in col..galaxies[row].len() {
                sum += manhattan_distance((galaxies[row][col], galaxies[row][other_col]));
            }
            for other_row in row + 1..galaxies.len() {
                for other_col in 0..galaxies[other_row].len() {
                    if row == other_row && col == other_col {
                        continue;
                    }
                    sum += manhattan_distance((galaxies[row][col], galaxies[other_row][other_col]));
                }
            }
        }
    }
    sum
}

fn manhattan_distance((a, b): ((usize, usize), (usize, usize))) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
