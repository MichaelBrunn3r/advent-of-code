use std::{collections::HashMap, usize};

use aoc::prelude::*;
use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn p(input: &str) -> (usize, usize) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut patterns_by_char = vec![Vec::new(); 6];
    patterns
        .split(", ")
        .map(|s| s.as_bytes())
        .for_each(|p| patterns_by_char[pattern_idx(p[0])].push(p));

    designs
        .par_split('\n')
        .map(|d| d.as_bytes())
        .filter(|d| d.len() > 0)
        .map(|d| num_possibilities(&patterns_by_char, d, &mut vec![u64::MAX; d.len() + 1]))
        .filter(|&n| n > 0)
        .map(|n| (1, n as usize))
        .reduce(|| (0,0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn num_possibilities<'d>(
    patterns_by_char: &Vec<Vec<&[u8]>>,
    design: &'d [u8],
    memo: &mut Vec<u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if memo[design.len()] != u64::MAX {
        return memo[design.len()];
    }

    let num_possibilities = patterns_by_char[pattern_idx(design[0])]
        .iter()
        .filter(|p| design.starts_with(p))
        .map(|p| num_possibilities(patterns_by_char, &design[p.len()..], memo))
        .sum();

    memo[design.len()] = num_possibilities;
    num_possibilities
}

fn pattern_idx(c: u8) -> usize {
    (c - b'0') as usize % 6
}