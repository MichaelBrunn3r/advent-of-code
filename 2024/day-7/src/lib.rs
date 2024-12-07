use aoc::prelude::*;
use itertools::Itertools;

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
    lines
        .iter()
        .filter_map(|(test, numbers)| {
            if is_valid_plus_mul(*test, &numbers, 0) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

pub fn p2(lines: &[(usize,Vec<(usize, usize)>)]) -> usize {
    lines
        .iter()
        .filter_map(|(test, numbers)| {
            if is_valid_plus_mul_concat(*test, &numbers, 0) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn is_valid_plus_mul(test: usize, numbers: &[(usize, usize)], sum: usize) -> bool {
    if numbers.len() == 0 {
        return sum == test;
    }

    if sum > test {
        return false;
    }

    return is_valid_plus_mul(test, &numbers[1..], sum*numbers[0].0)
        || is_valid_plus_mul(test, &numbers[1..], sum+numbers[0].0)
}

fn is_valid_plus_mul_concat(test: usize, numbers: &[(usize, usize)], sum: usize) -> bool {
    if numbers.len() == 0 {
        return sum == test;
    }

    return is_valid_plus_mul_concat(test, &numbers[1..], sum*numbers[0].0)
        || is_valid_plus_mul_concat(test, &numbers[1..], sum+numbers[0].0)
        || is_valid_plus_mul_concat(test, &numbers[1..], concat(sum, numbers[0].0, numbers[0].1))
}

fn concat(a: usize, b: usize, num_digits_b: usize) -> usize {
    a * (10usize.pow(num_digits_b as u32)) + b
}
