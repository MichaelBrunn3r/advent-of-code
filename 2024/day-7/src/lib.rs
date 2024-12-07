use core::num;

use aoc::prelude::*;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

const NUM_LINES: usize = 850;
// Operand digits: 1->3342, 2->1601, 3->1248


pub fn parse(input: &str) -> Vec<(usize,Vec<(usize, usize)>)> {
    let mut bytes = input.as_bytes();

    let mut equations = Vec::new();
    for _ in 0..NUM_LINES {
        let mut operands = Vec::new();

        let num_digits = position_colon(bytes);

        let test = bytes[..num_digits].parse_ascii_digits();
        bytes = &bytes[num_digits+2..];

        loop {
            let num_digits = if bytes[1] < b'0' {
                1
            } else if bytes[2] < b'0' {
                2
            } else {
                3
            };

            let operand = bytes[..num_digits].parse_ascii_digits();
            operands.push((operand, num_digits));
            
            let separator = bytes[num_digits];
            bytes = &bytes[num_digits+1..];
            if separator == b'\n' {
                break;
            }
        }

        equations.push((test, operands));
    }

    equations
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

// Test digits: 2->4, 3->33, 4->77, 5->91, 6->112, 7->116, 8->116, 9->120, 10->87, 11->60, 12->21, 13->10, 14->3
fn position_colon(bytes: &[u8]) -> usize {
    if bytes[9] == b':' {
        9
    } else if bytes[8] == b':' {
        8
    } else if bytes[7] == b':' {
        7
    } else if bytes[6] == b':' {
        6
    } else if bytes[5] == b':' {
        5
    } else if bytes[10] == b':' {
        10
    } else if bytes[4] == b':' {
        4
    } else if bytes[11] == b':' {
        11
    } else if bytes[3] == b':' {
        3
    } else if bytes[12] == b':' {
        12
    } else if bytes[13] == b':' {
        13
    } else if bytes[2] == b':' {
        2
    } else {
        14
    }
}