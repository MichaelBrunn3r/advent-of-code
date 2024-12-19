use std::collections::HashMap;

use aoc::prelude::*;
use itertools::Itertools;

pub fn p1(input: &str) -> usize {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").map(|s| s.as_bytes()).collect_vec();
    let mut memo = HashMap::new();

    designs
        .split("\n")
        .map(|d| d.as_bytes())
        .filter(|d| d.len() > 0)
        .filter(|d| is_possible(&patterns, d, &mut memo))
        .count()
}

pub fn p2(input: &str) -> usize {
    0
}

fn is_possible<'d>(patterns: &[&[u8]], design: &'d [u8], memo: &mut HashMap<&'d [u8], bool>) -> bool {
    if design.len() == 0 {
        return true;
    }

    if let Some(&is_possible) = memo.get(design) {
        return is_possible;
    }

    patterns.iter().filter(|p| design.starts_with(p)).any(|p| {
        let is_possible = is_possible(patterns, &design[p.len()..], memo);
        memo.insert(design, is_possible);
        is_possible
    })
}
