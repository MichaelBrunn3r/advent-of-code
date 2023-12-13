use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const ASH: u8 = b'.';
const ROCKS: u8 = b'#';

// 32810 too low
// 33728
// 34124 too high
pub fn part_1(input: &str) -> usize {
    let possible_reflections = input.split("\n\n").map(|block| {
        let mut cols = vec![0; block.lines().next().unwrap().len()];

        let rows = block
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

    let mut sum = 0;
    'blocks: for (rows, cols, row_reflections, col_reflections) in possible_reflections {
        'rows: for row_idx in row_reflections {
            let dist = (rows.len() - row_idx).min(row_idx);

            for i in 0..dist {
                if rows[row_idx - i - 1] != rows[row_idx + i] {
                    continue 'rows;
                }
            }

            sum += 100 * row_idx;
            continue 'blocks;
        }

        'cols: for col_idx in col_reflections {
            let dist = (cols.len() - col_idx).min(col_idx);

            for i in 0..dist {
                if cols[col_idx - i - 1] != cols[col_idx + i] {
                    continue 'cols;
                }
            }

            sum += col_idx;
            continue 'blocks;
        }
    }

    sum
}

pub fn part_2(input: &str) -> usize {
    0
}
