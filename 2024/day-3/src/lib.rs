use aoc::prelude::*;
use regex::Regex;

// ASSUMPTION 1: Maximum of 3 digits per operand
const MAX_MUL_LEN: usize = 12; // mul(123,456)
pub fn p1(input: &str) -> usize {
    let mut input = input.as_bytes();
    let mut sum = 0;
    
    while input.len() >= MAX_MUL_LEN {
        if walking_match(&mut input, b"mul(") {
            // PERF: Operands with 3 digits are most common in the input, by a large margin. By first determining the number
            //       of digits, the branch predictor can utilize this fact.
            let num_digits = if input[1] == b',' {
                1
            } else if input[2] == b',' {
                2
            } else if input[3] == b',' {
                3
            } else {
                continue;
            };

            // ASSUMPTION 2: There are only valid digits between 'mul(' and ','.
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

            // ASSUMPTION 3: There are only valid digits between ',' and ')'.
            let b = input.parse_n_ascii_digits(num_digits);
            input = &input[num_digits..];

            sum += a as usize * b as usize;
        }

        input = &input[1..];
    }

    sum
}

pub fn p2(input: &str) -> usize {
    let mut input = input.as_bytes();
    let mut sum = 0;

    let mut enabled = true;
    while input.len() >= MAX_MUL_LEN {
        // State enabled: Parse all 'mul(...)' and transition to state 'disabled' if we encounter a 'don't()'. Ignore all 'do()' as these won't change our state.
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

                // 'mul(' and 'don't()' have no overlap, so we can check for 'don't()' in every case.
                // The order of 'mul(' then 'don't()' is arbitrary, it was faster.
                if walking_match(&mut input, b"don't()") {
                    enabled = false;
                    break;
                }

                input = &input[1..];
            }
        // State disabled: Ignore all 'don't()' and 'mul(...)'. Transition to state 'enabled' if we encounter a 'do()'.
        } else {
            while input.len() >= MAX_MUL_LEN {
                if walking_match(&mut input, b"do()") {
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

pub fn p1_regex(input: &str) -> usize {
    let p = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for capture in p.captures_iter(input) {
        let a = capture.get(1).unwrap().as_str().as_bytes().parse_ascii_digits();
        let b = capture.get(2).unwrap().as_str().as_bytes().parse_ascii_digits();

        sum += a * b;
    }

    sum
}

pub fn p2_regex(input: &str) -> usize {
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

/// Check character by character if input matches expected. After a character is checked, remove it from the input.
/// NOTE: A raw poiner used as a cursor would have been nicer and spared us the bound checks, but also unsafe. Didn't feel like it today.
fn walking_match(input: &mut &[u8], expected: &[u8]) -> bool {
    for c in expected {
        if !(input[0] == *c) {
            return false;
        }
        *input = &input[1..];
    }

    true
}