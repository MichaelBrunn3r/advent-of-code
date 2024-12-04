#![feature(array_windows)]
#![feature(iter_map_windows)]

use aoc::prelude::*;
use itertools::Itertools;

const LINE_LENGTH: usize = 140 + 1;

pub fn part_1(input: &str) -> usize {
    let bytes = input.as_bytes();

    let horizontal: usize = bytes
        .array_windows::<4>()
        .filter(|&w| (w == b"XMAS" || w == b"SAMX"))
        .count();

    let vertical: usize = bytes
        .chunks_exact(LINE_LENGTH)
        .map_windows::<_,_,4>(|&lines| {
            let mut count = 0;
            let mut buf = [0;4];

            for i in 0..LINE_LENGTH {
                buf[0] = lines[0][i];
                buf[1] = lines[1][i];
                buf[2] = lines[2][i];
                buf[3] = lines[3][i];

                count += (buf == *b"XMAS" || buf == *b"SAMX") as usize;
            }

            count
        })
        .sum();

    let diagonal = bytes
        .chunks_exact(LINE_LENGTH)
        .tuple_windows::<(&[u8], &[u8], &[u8], &[u8])>()
        .map(|lines| {
            let mut count = 0;
            let mut buf_down = [0;4]; // "\"
            let mut buf_up = [0;4];   // "/"
            
            for i in 2..lines.0.len()-2 {
                buf_down[0] = lines.0[i-2];
                buf_down[1] = lines.1[i-1];
                buf_down[2] = lines.2[i];
                buf_down[3] = lines.3[i+1];

                count += (buf_down == *b"XMAS" || buf_down == *b"SAMX") as usize;

                buf_up[0] = lines.3[i-2];
                buf_up[1] = lines.2[i-1];
                buf_up[2] = lines.1[i];
                buf_up[3] = lines.0[i+1];

                count += (buf_up == *b"XMAS" || buf_up == *b"SAMX") as usize;
            }
            count
        })
        .sum::<usize>();

    horizontal + vertical + diagonal
}

pub fn part_2(input: &str) -> usize {
    input
        .as_bytes()
        .chunks_exact(LINE_LENGTH)
        .tuple_windows::<(_,_,_)>()
        .map(|lines| {
            let mut count = 0;
            let mut buf_down = [0;3]; // "\"
            let mut buf_up = [b'A';3];   // "/"

            for i in 1..lines.0.len()-2 {
                buf_down[0] = lines.0[i-1];
                buf_down[1] = lines.1[i];
                buf_down[2] = lines.2[i+1];

                buf_up[0] = lines.2[i-1];
                buf_up[2] = lines.0[i+1];

                count += ((buf_down == *b"MAS" || buf_down == *b"SAM")
                    && (buf_up == *b"MAS" || buf_up == *b"SAM")) as usize;
            }
            count
        })
        .sum()
}
