use aoc::prelude::*;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn p(input: &str) -> (usize, usize) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let mut trie = vec![Node::new()];
    patterns.split(", ").for_each(|p| {
        let mut i = 0;
        p.bytes().map(hash).for_each(|color| {
            if trie[i].next[color] == 0 {
                trie[i].next[color] = trie.len();
                i = trie.len();
                trie.push(Node::new());
            } else {
                i = trie[i].next[color]
            }
        });
        trie[i].set_towel();
    });

    designs
        .par_split('\n')
        .map(|d| d.as_bytes())
        .filter(|d| d.len() > 0)
        .map(|d| num_possibilities(&trie, d, &mut vec![u64::MAX; d.len() + 1]))
        .filter(|&n| n > 0)
        .map(|n| (1, n as usize))
        .reduce(|| (0,0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn num_possibilities<'d>(
    trie: &[Node],
    design: &'d [u8],
    memo: &mut Vec<u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if memo[design.len()] != u64::MAX {
        return memo[design.len()];
    }

    let mut sum = 0;
    let mut n = &trie[0];
    for (i, &c) in design.iter().enumerate() {
        let color = hash(c);
        if n.next[color] == 0 {
            break;
        }

        n = &trie[n.next[color]];

        if n.is_towel() {
            sum += num_possibilities(trie, &design[i+1..], memo);
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
struct Node {
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
