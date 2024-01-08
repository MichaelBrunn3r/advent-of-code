use aoc::prelude::*;
use itertools::Itertools;

const ROCKS: u8 = b'#';

pub fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::parse).collect_vec()
}

pub fn part_1(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let row_reflections = pattern.rows.iter().duplicate_positions();
            let col_reflections = pattern.cols.iter().duplicate_positions();
            (pattern, row_reflections, col_reflections)
        })
        .map(|(pattern, row_reflections, col_reflections)| {
            for row_idx in row_reflections {
                if pattern.rows.partialy_reflects_at(row_idx) {
                    return 100 * row_idx;
                }
            }

            for col_idx in col_reflections {
                if pattern.cols.partialy_reflects_at(col_idx) {
                    return col_idx;
                }
            }

            0
        })
        .sum()
}

pub fn part_2(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let row_reflections = duplicate_positions_or_smudged(&pattern.rows);
            let col_reflections = duplicate_positions_or_smudged(&pattern.cols);
            (pattern, row_reflections, col_reflections)
        })
        .enumerate()
        .map(|(_, (pattern, row_reflections, col_reflections))| {
            for row_idx in row_reflections {
                if reflection_with_smudge_at(&pattern.rows, row_idx) {
                    return 100 * row_idx;
                }
            }

            for col_idx in col_reflections {
                if reflection_with_smudge_at(&pattern.cols, col_idx) {
                    return col_idx;
                }
            }

            0
        })
        .sum()
}

fn duplicate_positions_or_smudged(lines: &[usize]) -> Vec<usize> {
    let mut reflections = vec![];

    for ((_, prev), (curr_idx, curr)) in lines.iter().enumerate().tuple_windows() {
        let dist = prev.hamming_distance(curr);

        if dist <= 1 {
            reflections.push(curr_idx);
        }
    }

    reflections
}

fn reflection_with_smudge_at(lines: &[usize], idx: usize) -> bool {
    let dist = (lines.len() - idx).min(idx);

    let mut has_smudge = false;
    for i in 0..dist {
        let dist = lines[idx - i - 1].hamming_distance(&lines[idx + i]);

        if dist > 0 {
            if !has_smudge && dist == 1 {
                has_smudge = true;
            } else {
                return false;
            }
        }
    }

    has_smudge
}

#[derive(Debug)]
pub struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Pattern {
    fn parse(section: &str) -> Pattern {
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
