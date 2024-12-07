use aoc::prelude::*;
use itertools::Itertools;

const NUM_LINES: usize = 850;

pub fn p1(input: &str) -> usize {
    let bytes = input.as_bytes();

    bytes
        .split(|&c| c == b'\n')
        .take(NUM_LINES)
        .map(|mut l| {
            let mut parts = l.split(|&c| c == b' ');
            let test = parts.next().unwrap().split_last().unwrap().1.parse_ascii_digits();
            let numbers = parts.map(|p| p.parse_ascii_digits()).collect_vec();
            (test, numbers)
        })
        .filter_map(|(test, numbers)| {
            if is_valid_plus_mul(test, &numbers, 0) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    let bytes = input.as_bytes();

    bytes
        .split(|&c| c == b'\n')
        .take(NUM_LINES)
        .map(|l| {
            let mut parts = l.split(|&c| c == b' ');
            let test = parts.next().unwrap().split_last().unwrap().1.parse_ascii_digits();
            let numbers = parts.map(|p| p.parse_ascii_digits()).collect_vec();
            (test, numbers)
        })
        .filter_map(|(test, numbers)| {
            if is_valid_plus_mul_concat(test, &numbers, 0) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn is_valid_plus_mul(test: usize, numbers: &[usize], sum: usize) -> bool {
    if numbers.len() == 0 {
        return sum == test;
    }

    if sum > test {
        return false;
    }

    return is_valid_plus_mul(test, &numbers[1..], sum*numbers[0])
        || is_valid_plus_mul(test, &numbers[1..], sum+numbers[0])
}

fn is_valid_plus_mul_concat(test: usize, numbers: &[usize], sum: usize) -> bool {
    if numbers.len() == 0 {
        return sum == test;
    }

    return is_valid_plus_mul_concat(test, &numbers[1..], sum*numbers[0])
        || is_valid_plus_mul_concat(test, &numbers[1..], sum+numbers[0])
        || is_valid_plus_mul_concat(test, &numbers[1..], concat(sum, numbers[0]))
}

fn concat(mut a: usize, b: usize) -> usize {
    let num_digits = ((b as f64).log10() + 1.0).floor() as usize;
    for _ in 0..num_digits {
        a *= 10;
    }
    a + b
}
