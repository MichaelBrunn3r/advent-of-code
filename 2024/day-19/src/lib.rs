use std::collections::HashMap;

use aoc::prelude::*;
use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn p(input: &str) -> (usize, usize) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").map(|s| s.as_bytes()).collect_vec();

    designs
        .par_split('\n')
        .map(|d| d.as_bytes())
        .filter(|d| d.len() > 0)
        .map(|d| num_possibilities(&patterns, d, &mut FxHashMap::default()))
        .filter(|&n| n > 0)
        .map(|n| (1, n))
        .reduce(|| (0,0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn num_possibilities<'d>(
    patterns: &[&[u8]],
    design: &'d [u8],
    memo: &mut FxHashMap<&'d [u8], usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if let Some(&num_possibilities) = memo.get(design) {
        return num_possibilities;
    }

    patterns
        .iter()
        .filter(|p| design.starts_with(p))
        .map(|p| {
            let design = &design[p.len()..];
            let num_possibilities = num_possibilities(patterns, design, memo);
            memo.insert(design, num_possibilities);
            num_possibilities
        })
        .sum()
}
