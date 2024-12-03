use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const MAX_MUL_LEN: usize = 12; // mul(123,456)
const MIN_MUL_LEN: usize = 8;  // mul(1,2)
 
pub fn part_1(input: &str) -> usize {
    let mut input = input.as_bytes();
    let mut sum = 0;
    
    while input.len() >= MAX_MUL_LEN {
        if walking_match(&mut input, b"mul(") {
            let num_digits = if input[1] == b',' {
                1
            } else if input[2] == b',' {
                2
            } else if input[3] == b',' {
                3
            } else {
                continue;
            };

            let a = input.parse_n_ascii_digits(num_digits);
            input = &input[num_digits + 1..];

            let num_digits = if input[1] == b')' {
                1
            } else if input[2] == b')' {
                2
            } else if input[3] == b')' {
                3
            } else {
                continue;
            };

            let b = input.parse_n_ascii_digits(num_digits);
            input = &input[num_digits..];

            sum += a as usize * b as usize;
        }

        input = &input[1..];
    }

    sum
}

pub fn part_2(input: &str) -> usize {
    let mut input = input.as_bytes();
    let mut sum = 0;

    let mut enabled = true;
    while input.len() >= MAX_MUL_LEN {
        if enabled {
            while input.len() >= MAX_MUL_LEN {
                if walking_match(&mut input, b"mul(") {
                    let num_digits = if input[1] == b',' {
                        1
                    } else if input[2] == b',' {
                        2
                    } else if input[3] == b',' {
                        3
                    } else {
                        continue;
                    };

                    let a = input.parse_n_ascii_digits(num_digits);
                    input = &input[num_digits + 1..];

                    let num_digits = if input[1] == b')' {
                        1
                    } else if input[2] == b')' {
                        2
                    } else if input[3] == b')' {
                        3
                    } else {
                        continue;
                    };

                    let b = input.parse_n_ascii_digits(num_digits);
                    input = &input[num_digits..];

                    sum += a as usize * b as usize;
                }

                if walking_match(&mut input, b"don't()") {
                    enabled = false;
                    break;
                }

                input = &input[1..];
            }
        } else {
            while input.len() >= MAX_MUL_LEN {
                if walking_match(&mut input, b"do(") {
                    input = &input[1..];
                    enabled = true;
                    break;
                }

                input = &input[1..];
            }
        }
    }

    sum
}

pub fn part_1_regex(input: &str) -> usize {
    let p = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for capture in p.captures_iter(input) {
        let a = capture.get(1).unwrap().as_str().as_bytes().parse_ascii_digits();
        let b = capture.get(2).unwrap().as_str().as_bytes().parse_ascii_digits();

        sum += a * b;
    }

    sum
}

pub fn part_2_regex(input: &str) -> usize {
    let p = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<a>\d+),(?<b>\d+)\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for capture in p.captures_iter(input) {
        if capture.name("dont") != None {
            enabled = false;
        } else if capture.name("do") != None {
            enabled = true;
        } else if enabled {
            let a = capture.name("a").unwrap().as_str().as_bytes().parse_ascii_digits();
            let b = capture.name("b").unwrap().as_str().as_bytes().parse_ascii_digits();
            sum += a * b;
        }
    }
    
    sum
}

fn walking_match(input: &mut &[u8], expected: &[u8]) -> bool {
    for c in expected {
        if !(input[0] == *c) {
            return false;
        }
        *input = &input[1..];
    }

    true
}