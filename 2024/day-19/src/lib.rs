use aoc::prelude::*;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
// 1: 4, 2: 23, 3: 120, 4: 100, 5: 80, 6: 60, 7: 39, 8: 20

pub fn parse<'i>(input: &'i str) -> (Vec<Node>, &'i [u8]) {
    let mut trie = Vec::with_capacity(850);
    trie.push(Node::new());

    let mut crs = input.as_ptr();
    let mut idx = 0;
    loop {
        let c = crs.take();
        if c < b'a' {
            trie[idx].set_towel();
            idx = 0;

            if crs.take() == b'\n' {
                break;
            } else {    
                continue;
            }
        }

        let color = hash(c);
        idx = if trie[idx].next[color] == 0 {
            trie[idx].next[color] = trie.len();
            trie.push(Node::new());
            trie.len() - 1
        } else {
            trie[idx].next[color]
        }
    }

    (trie, &input.as_bytes()[unsafe{crs.offset_from(input.as_ptr())} as usize..])
}

pub fn p(patterns: &[Node], designs: &[u8]) -> (usize, usize) {
    designs
        .par_split(|&b|b == b'\n')
        .filter(|d| d.len() > 0)
        .map(|d| num_possibilities(&patterns, d, &mut [u64::MAX; 65]))
        .filter(|&n| n > 0)
        .map(|n| (1, n as usize))
        .reduce(|| (0,0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn num_possibilities<'d>(
    patterns: &[Node],
    design: &'d [u8],
    memo: &mut [u64],
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if memo[design.len()] != u64::MAX {
        return memo[design.len()];
    }

    let mut sum = 0;
    let mut n = &patterns[0];
    for (i, &c) in design.iter().enumerate() {
        let color = hash(c);
        if n.next[color] == 0 {
            break;
        }

        n = &patterns[n.next[color]];

        if n.is_towel() {
            sum += num_possibilities(patterns, &design[i+1..], memo);
        }
    }

    memo[design.len()] = sum;
    sum
}

/// Calculates a perfect hash for a color in the range of 0..=5.
///
/// |color|hash|
/// |-|-|
/// |r|0|
/// |g|1|
/// |b|2|
/// |u|3|
/// ||4|
/// |w|5|
fn hash(c: u8) -> usize {
    (c - b'0') as usize % 6
}

#[derive(Debug)]
pub struct Node {
    next: [usize; 6],
}

impl Node {
    /// [`hash`] does not generate the value 4 so we can reuse it as a flag
    const IS_TOWEL_FLAG_IDX: usize = 4;

    fn new() -> Self {
        Self { next: [0; 6] }
    }

    fn is_towel(&self) -> bool {
        self.next[Self::IS_TOWEL_FLAG_IDX] != 0
    }

    fn set_towel(&mut self) {
        self.next[Self::IS_TOWEL_FLAG_IDX] = 1;
    }
}
