use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const ASH: u8 = b'.';
const ROCKS: u8 = b'#';

pub fn part_1(input: &str) -> usize {
    let blocks = input.split("\n\n").map(|block| {
        let num_cols = block.lines().next().unwrap().len();
        let (rows, cols) = parse_block(block.lines(), num_cols);

        let mut row_reflections = vec![];
        rows.iter()
            .enumerate()
            .tuple_windows()
            .for_each(|((_, prev), (curr_idx, curr))| {
                if prev == curr {
                    row_reflections.push(curr_idx);
                }
            });

        let mut col_reflections = vec![];
        cols.iter()
            .enumerate()
            .tuple_windows()
            .for_each(|((_, prev), (curr_idx, curr))| {
                if prev == curr {
                    col_reflections.push(curr_idx);
                }
            });

        (rows, cols, row_reflections, col_reflections)
    });

    blocks
        .map(|(rows, cols, row_reflections, col_reflections)| {
            calc_block_value(rows, cols, row_reflections, col_reflections)
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

fn calc_block_value(
    rows: Vec<usize>,
    cols: Vec<usize>,
    row_reflections: Vec<usize>,
    col_reflections: Vec<usize>,
) -> usize {
    for row_idx in row_reflections {
        if is_reflection(row_idx, &rows) {
            return 100 * row_idx;
        }
    }

    for col_idx in col_reflections {
        if is_reflection(col_idx, &cols) {
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

fn parse_block<'a>(
    lines: impl Iterator<Item = &'a str>,
    num_cols: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut cols = vec![0; num_cols];

    let rows = lines
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

    (rows, cols)
}
