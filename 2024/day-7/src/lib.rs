use core::num;

use aoc::prelude::*;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

const NUM_LINES: usize = 850;

pub fn parse(input: &str) -> Vec<(usize,Vec<(usize, usize)>)> {
    input
        .as_bytes()
        .split(|&c| c == b'\n')
        .take(NUM_LINES)
        .map(|l| {
            let mut parts = l.split(|&c| c == b' ');
            let test = parts.next().unwrap().split_last().unwrap().1.parse_ascii_digits();
            let numbers = parts.map(|p| (p.parse_ascii_digits(), p.len())).collect_vec();
            (test, numbers)
        })
        .collect_vec()
}

pub fn p1(lines: &[(usize,Vec<(usize, usize)>)]) -> usize {
    let mut stack = Vec::new();
    lines
        .iter()
        .filter_map(|(test, numbers)| {
            stack.clear();
            stack.push((numbers.len()-1, *test));

            while let Some((i, rest)) = stack.pop() {
                if i == 0 {
                    if rest == numbers[i].0 {
                        return Some(*test);
                    }
                    continue;
                }
                
                if rest >= numbers[i].0 {
                    stack.push((i-1, rest - numbers[i].0));
                } else {
                    continue;
                }
                
                if rest % numbers[i].0 == 0 {
                    stack.push((i-1, rest / numbers[i].0));
                }
            }

            None
        })
        .sum()
}

pub fn p2(lines: &[(usize,Vec<(usize, usize)>)]) -> usize {
    let mut stack = Vec::with_capacity(11);
    lines
        .iter()
        .filter_map(|(test, numbers)| {
            stack.clear();
            stack.push((numbers.len()-1, *test));

            while let Some((i, rest)) = stack.pop() {
                if i == 0 {
                    if rest == numbers[i].0 {
                        return Some(*test);
                    }
                    continue;
                }
                
                if rest >= numbers[i].0 {
                    stack.push((i-1, rest - numbers[i].0));
                } else {
                    continue;
                }
                
                if rest % numbers[i].0 == 0 {
                    stack.push((i-1, rest / numbers[i].0));
                }

                let div = rest / LUT_POW_10[numbers[i].1];
                if div * LUT_POW_10[numbers[i].1] + numbers[i].0 == rest {
                    stack.push((i-1, div));
                }
            }

            None
        })
        .sum()
}

const LUT_POW_10: [usize; 5] = [1, 10, 100, 1000, 10000];
