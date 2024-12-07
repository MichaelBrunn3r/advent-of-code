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
    let mut stack = Vec::with_capacity(11);
    lines
        .iter()
        .filter_map(|(test, numbers)| {
            stack.clear();
            stack.push((1, numbers[0].0));

            while let Some((i, sum)) = stack.pop() {
                let add = sum + numbers[i].0;
                let mul = sum * numbers[i].0;

                if i >= numbers.len()-1 {
                    if add == *test || mul == *test {
                        return Some(*test);
                    }
                    continue;
                }

                if add <= *test {
                    stack.push((i+1, add));
                }

                if mul <= *test {
                    stack.push((i+1, mul));
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
            stack.push((1, numbers[0].0));

            while let Some((i, sum)) = stack.pop() {

                if i >= numbers.len()-1 {
                    let add = sum + numbers[i].0;
                    let mul = sum * numbers[i].0;
                    let conc = concat(sum , numbers[i].0, numbers[i].1);
                    
                    if add == *test || mul == *test || conc == *test {
                        return Some(test);
                    }
                    continue;
                }

                let add = sum + numbers[i].0;
                if add >= *test + 1 {
                    continue;
                }
                stack.push((i+1, add));

                let mul = sum * numbers[i].0;
                if mul >= *test + 10 {
                    continue;
                }
                stack.push((i+1, mul));


                let conc = concat(sum , numbers[i].0, numbers[i].1);
                stack.push((i+1, conc));
            }

            None
        })
        .sum()
}

fn concat(a: usize, b: usize, num_digits_b: usize) -> usize {
    a * (10usize.pow(num_digits_b as u32)) + b
}
