#![feature(array_windows)]

use aoc::prelude::*;
use itertools::Itertools;

const NUM_LINES: usize = 140;

// 2449 too low
// 2490 too low
pub fn part_1(input: &str) -> usize {
    let horizontal: usize = input.split("\n")
        .take(NUM_LINES)
        .map(|l| {
            l.as_bytes()
                .array_windows()
                .map(|x: &[u8; 4]| {
                    (x == b"XMAS" || x == b"SAMX") as usize
                })
                .sum::<usize>()
        })
        .sum();

    let vertical: usize = input.split("\n")
        .take(NUM_LINES)
        .tuple_windows()
        .map(|lines: (&str, &str, &str, &str)| {
            let mut num_xmas = 0;
            let mut buf = [0,0,0,0];
            for i in 0..lines.0.len() {
                buf[0] = lines.0.as_bytes()[i];
                buf[1] = lines.1.as_bytes()[i];
                buf[2] = lines.2.as_bytes()[i];
                buf[3] = lines.3.as_bytes()[i];

                num_xmas += (buf == *b"XMAS" || buf == *b"SAMX") as usize;
            }
            num_xmas
        })
        .sum();

    let diagonal: usize = input.split("\n")
        .take(NUM_LINES)
        .tuple_windows()
        .map(|lines: (&str, &str, &str, &str)| {
            let mut num_xmas = 0;
            let mut buf_down = [0,0,0,0]; // "\"
            let mut buf_up = [0,0,0,0];   // "/"
            for i in 2..lines.0.len()-1 {
                buf_down[0] = lines.0.as_bytes()[i-2];
                buf_down[1] = lines.1.as_bytes()[i-1];
                buf_down[2] = lines.2.as_bytes()[i];
                buf_down[3] = lines.3.as_bytes()[i+1];

                num_xmas += (buf_down == *b"XMAS" || buf_down == *b"SAMX") as usize;

                buf_up[0] = lines.3.as_bytes()[i-2];
                buf_up[1] = lines.2.as_bytes()[i-1];
                buf_up[2] = lines.1.as_bytes()[i];
                buf_up[3] = lines.0.as_bytes()[i+1];

                num_xmas += (buf_up == *b"XMAS" || buf_up == *b"SAMX") as usize;
            }
            num_xmas
        })
        .sum();

    horizontal + vertical + diagonal
}

pub fn part_2(input: &str) -> usize {
    input.split("\n")
        .take(NUM_LINES)
        .tuple_windows()
        .map(|lines: (&str, &str, &str)| {
            let mut num_x_mas = 0;
            let mut buf_down = [0,0,0]; // "\"
            let mut buf_up = [0,0,0];   // "/"
            for i in 1..lines.0.len()-1 {
                buf_down[0] = lines.0.as_bytes()[i-1];
                buf_down[1] = lines.1.as_bytes()[i];
                buf_down[2] = lines.2.as_bytes()[i+1];

                buf_up[0] = lines.2.as_bytes()[i-1];
                buf_up[1] = lines.1.as_bytes()[i];
                buf_up[2] = lines.0.as_bytes()[i+1];

                num_x_mas += ((buf_down == *b"MAS" || buf_down == *b"SAM")
                    && (buf_up == *b"MAS" || buf_up == *b"SAM")) as usize;
            }
            num_x_mas
        })
        .sum()
}
