use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const ASH: u8 = b'.';
const ROCKS: u8 = b'#';

pub fn part_1(input: &str) -> usize {
    let patterns = input.split("\n\n").map(|section| {
        let pattern = Pattern::parse(section);

        let row_axes = pattern.rows.iter().duplicate_positions();
        let col_axes = pattern.cols.iter().duplicate_positions();

        (pattern, row_axes, col_axes)
    });

    patterns
        .map(|(pattern, row_axes, col_axes)| calc_patterns_value(pattern, row_axes, col_axes))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

fn calc_patterns_value(pattern: Pattern, row_axes: Vec<usize>, col_axes: Vec<usize>) -> usize {
    for row_idx in row_axes {
        if is_reflection(row_idx, &pattern.rows) {
            return 100 * row_idx;
        }
    }

    for col_idx in col_axes {
        if is_reflection(col_idx, &pattern.cols) {
            return col_idx;
        }
    }

    0
}

fn is_reflection(line_idx: usize, lines: &Vec<usize>) -> bool {
    let dist = (lines.len() - line_idx).min(line_idx);

    for i in 0..dist {
        if lines[line_idx - i - 1] != lines[line_idx + i] {
            return false;
        }
    }

    return true;
}

struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Pattern {
    fn parse<'a>(section: &str) -> Pattern {
        let num_cols = section.lines().next().unwrap().len();
        let mut cols = vec![0; num_cols];

        let rows = section
            .lines()
            .enumerate()
            .map(|(row_idx, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(col_idx, c)| {
                        let is_rocks = (c == ROCKS) as usize;
                        cols[col_idx] += is_rocks << row_idx;
                        is_rocks << col_idx
                    })
                    .sum::<usize>()
            })
            .collect_vec();

        Pattern { rows, cols }
    }
}
