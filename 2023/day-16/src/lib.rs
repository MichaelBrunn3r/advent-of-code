#![allow(dead_code)]
#![feature(stdsimd)]
use std::arch::x86_64::{
    _mm256_cmpeq_epi8, _mm256_loadu_epi8, _mm256_movemask_epi8, _mm256_set1_epi8, _mm_movemask_epi8,
};

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const EMPTY: u8 = b'.';
const MIRROR_R: u8 = b'/';
const MIRROR_L: u8 = b'\\';
const SPLIT_V: u8 = b'|';
const SPLIT_H: u8 = b'-';

// Calc vert dir changes:                  12.483 µs
// Calc part of vert dir changes with AVX:  2.519 µs
pub fn part_1(input: &str) -> usize {
    let width = input.find('\n').unwrap();

    let mirror_r = unsafe { _mm256_set1_epi8(MIRROR_R as i8) };
    let mirror_l = unsafe { _mm256_set1_epi8(MIRROR_L as i8) };
    let split_v = unsafe { _mm256_set1_epi8(SPLIT_V as i8) };

    let vert_dir_changes = input
        .as_bytes()
        .chunks_exact(32)
        .map(|line| {
            let chunk = unsafe { _mm256_loadu_epi8(line.as_ptr() as *const _) };
            let is_mirror_r =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_r)) } as u32;
            let is_mirror_l =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_l)) } as u32;
            let is_splitter_v =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, split_v)) } as u32;

            let mut dir_changes = (is_mirror_r | is_mirror_l | is_splitter_v) as u128;

            let chunk = unsafe { _mm256_loadu_epi8(line.as_ptr().add(32) as *const _) };
            let is_mirror_r =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_r)) } as u32;
            let is_mirror_l =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_l)) } as u32;
            let is_split_v =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, split_v)) } as u32;

            dir_changes |= ((is_mirror_r | is_mirror_l | is_split_v) as u128) << 32;

            let chunk = unsafe { _mm256_loadu_epi8(line.as_ptr().add(64) as *const _) };
            let is_mirror_r =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_r)) } as u32;
            let is_mirror_l =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_l)) } as u32;
            let is_split_v =
                unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, split_v)) } as u32;

            dir_changes |= ((is_mirror_r | is_mirror_l | is_split_v) as u128) << 64;

            let chunk = unsafe { _mm256_loadu_epi8(line.as_ptr().add(96) as *const _) };
            let is_mirror_r = unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_r)) };
            let is_mirror_l = unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, mirror_l)) };
            let is_split_v = unsafe { _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, split_v)) };

            dir_changes |= ((is_mirror_r | is_mirror_l | is_split_v) as u128) << 96;
            dir_changes &= 0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111;

            dir_changes
        })
        .collect_vec();

    // vert_dir_changes
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, dir_change)| {
    //         println!(
    //             "{}",
    //             input
    //                 .lines()
    //                 .nth(i)
    //                 .unwrap()
    //                 .chars()
    //                 .rev()
    //                 .collect::<String>()
    //         );
    //         println!("{:0110b}", dir_change);
    //     });

    vert_dir_changes
        .iter()
        .map(|&row| row.count_ones() as usize)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}
