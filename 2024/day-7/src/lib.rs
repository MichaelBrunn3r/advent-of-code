use core::slice;
use aoc::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const NUM_LINES: usize = 850;

pub fn parse(input: &str) -> Vec<(usize,Vec<(usize, usize)>)> {
    let mut crs = input.as_ptr();

    let mut equations = Vec::with_capacity(NUM_LINES);
    for _ in 0..NUM_LINES {
        let mut operands = Vec::with_capacity(12);
        let num_digits = offset_colon(crs);

        let test = unsafe{slice::from_raw_parts(crs, num_digits)}.parse_ascii_digits();
        unsafe{crs = crs.add(num_digits+2)};

        loop {
            let num_digits = offset_operand_terminator(crs);
            let operand = unsafe{slice::from_raw_parts(crs, num_digits)}.parse_ascii_digits();
            operands.push((operand, num_digits));
            
            let separator = unsafe{*crs.offset(num_digits as isize)};
            unsafe{crs = crs.add(num_digits+1)};
            if separator == b'\n' {
                break;
            }
        }

        equations.push((test, operands));
    }

    equations
}

pub fn p(equations: &[(usize,Vec<(usize, usize)>)]) -> (usize, usize) {
    equations
        .par_iter()
        .map(|(test, numbers)| {
            let mut stack = Vec::with_capacity(11);
            stack.push((false, numbers.len()-1, *test));

            while let Some((used_concat, i, rest)) = stack.pop() {
                if i == 0 {
                    if rest == numbers[i].0 {
                        return if used_concat {
                            (0, *test)
                        } else {
                            (*test, *test)
                        };
                    }
                    continue;
                }
                
                if rest >= numbers[i].0 {
                    stack.push((used_concat, i-1, rest - numbers[i].0));
                } else {
                    continue;
                }
                
                if rest % numbers[i].0 == 0 {
                    stack.push((used_concat, i-1, rest / numbers[i].0));
                }

                let div = rest / LUT_POW_10[numbers[i].1];
                if div * LUT_POW_10[numbers[i].1] + numbers[i].0 == rest {
                    stack.push((true, i-1, div));
                }
            }

            (0,0)
        })
        .reduce(|| (0,0), |a,b| (a.0 + b.0, a.1 + b.1))
}

const LUT_POW_10: [usize; 5] = [1, 10, 100, 1000, 10000];

// Test digits: 2->4, 3->33, 4->77, 5->91, 6->112, 7->116, 8->116, 9->120, 10->87, 11->60, 12->21, 13->10, 14->3
fn offset_colon(crs: *const u8) -> usize {
    if unsafe{*crs.offset(9)} == b':' {
        9
    } else if unsafe{*crs.offset(8)} == b':' {
        8
    } else if unsafe{*crs.offset(7)} == b':' {
        7
    } else if unsafe{*crs.offset(6)} == b':' {
        6
    } else if unsafe{*crs.offset(5)} == b':' {
        5
    } else if unsafe{*crs.offset(10)} == b':' {
        10
    } else if unsafe{*crs.offset(4)} == b':' {
        4
    } else if unsafe{*crs.offset(11)} == b':' {
        11
    } else if unsafe{*crs.offset(3)} == b':' {
        3
    } else if unsafe{*crs.offset(12)} == b':' {
        12
    } else if unsafe{*crs.offset(13)} == b':' {
        13
    } else if unsafe{*crs.offset(2)} == b':' {
        2
    } else {
        14
    }
}

// Operand digits: 1->3342, 2->1601, 3->1248
fn offset_operand_terminator(crs: *const u8) -> usize {
    if unsafe{*crs.offset(1)} < b'0' {
        1
    } else if unsafe{*crs.offset(2)} < b'0' {
        2
    } else {
        3
    }
}