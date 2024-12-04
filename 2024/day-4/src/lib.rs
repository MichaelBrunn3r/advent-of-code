#![feature(array_windows)]

use aoc::prelude::*;
use itertools::Itertools;

const SIDE_LENGTH: usize = 140;


pub fn part_1(input: &str) -> usize {
    let horizontal: usize = input
        .as_bytes()
        .array_windows()
        .map(|w: &[u8; 4]| {
            (w == b"XMAS" || w == b"SAMX") as usize
        })
        .sum();

    let vertical: usize = input
        .as_bytes()
        .chunks_exact(SIDE_LENGTH + 1)
        .tuple_windows::<(&[u8], &[u8], &[u8], &[u8])>()
        .map(|lines| {
            let mut num_xmas = 0;
            let mut buf = [0,0,0,0];
            for i in 0..lines.0.len() {
                buf[0] = lines.0[i];
                buf[1] = lines.1[i];
                buf[2] = lines.2[i];
                buf[3] = lines.3[i];

                num_xmas += (buf == *b"XMAS" || buf == *b"SAMX") as usize;
            }
            num_xmas
        })
        .sum();

    let diagonal: usize = input
        .as_bytes()
        .chunks_exact(SIDE_LENGTH + 1)
        .tuple_windows::<(&[u8], &[u8], &[u8], &[u8])>()
        .map(|lines| {
            let mut num_xmas = 0;
            let mut buf_down = [0,0,0,0]; // "\"
            let mut buf_up = [0,0,0,0];   // "/"
            for i in 2..lines.0.len()-2 {
                buf_down[0] = lines.0[i-2];
                buf_down[1] = lines.1[i-1];
                buf_down[2] = lines.2[i];
                buf_down[3] = lines.3[i+1];

                num_xmas += (buf_down == *b"XMAS" || buf_down == *b"SAMX") as usize;

                buf_up[0] = lines.3[i-2];
                buf_up[1] = lines.2[i-1];
                buf_up[2] = lines.1[i];
                buf_up[3] = lines.0[i+1];

                num_xmas += (buf_up == *b"XMAS" || buf_up == *b"SAMX") as usize;
            }
            num_xmas
        })
        .sum();

    horizontal + vertical + diagonal
}

pub fn part_2(input: &str) -> usize {
    input
        .as_bytes()
        .chunks_exact(SIDE_LENGTH + 1)
        .tuple_windows::<(&[u8], &[u8], &[u8])>()
        .map(|lines| {
            let mut num_x_mas = 0;
            let mut buf_down = [0,0,0]; // "\"
            let mut buf_up = [0,0,0];   // "/"
            for i in 1..lines.0.len()-2 {
                buf_down[0] = lines.0[i-1];
                buf_down[1] = lines.1[i];
                buf_down[2] = lines.2[i+1];

                buf_up[0] = lines.2[i-1];
                buf_up[1] = lines.1[i];
                buf_up[2] = lines.0[i+1];

                num_x_mas += ((buf_down == *b"MAS" || buf_down == *b"SAM")
                    && (buf_up == *b"MAS" || buf_up == *b"SAM")) as usize;
            }
            num_x_mas
        })
        .sum()
}
