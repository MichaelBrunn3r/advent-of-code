#![feature(slice_split_once)]
use aoc::prelude::*;
use core::str;
use std::{cmp::Reverse, collections::BinaryHeap};

const INNER_SIZE: usize = 71;
const SIZE: usize = INNER_SIZE + 2;
const START: usize = SIZE + 1;
const EXIT: usize = SIZE * (SIZE - 1) - 2;

pub fn p1(input: &str) -> usize {
    let mut grid = [b'.'; SIZE * SIZE];
    (0..SIZE).for_each(|i| {
        grid[i] = b'#';
        grid[i * SIZE] = b'#';
        grid[i * SIZE] = b'#';
        grid[i * SIZE + (SIZE - 1)] = b'#';
    });
    ((SIZE - 1) * SIZE..SIZE * SIZE).for_each(|i| grid[i] = b'#');

    // grid[START] = b'S';
    // grid[EXIT] = b'E';
    // grid.chunks_exact(SIZE).for_each(|c| {
    //     println!("{}", unsafe { str::from_utf8_unchecked(&c) });
    // });

    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .take(1024)
        .map(|l| {
            let (x, y) = l.split_once(|&b| b == b',').unwrap();
            xy(x.parse_ascii_digits(), y.parse_ascii_digits())
        })
        .for_each(|p| {
            grid[p.x as usize + SIZE + 1 + p.y as usize * SIZE] = b'#';
        });

    // grid.chunks_exact(SIZE).for_each(|c| {
    //     println!("{}", unsafe { str::from_utf8_unchecked(&c) });
    // });

    let mut best_cost = [usize::MAX; SIZE * SIZE];
    let mut stack = BinaryHeap::new();
    stack.push(Reverse((0usize, START)));
    while let Some(Reverse((current_cost, current_pos))) = stack.pop() {
        if current_pos == EXIT {
            return current_cost;
        }

        if current_cost >= best_cost[current_pos] {
            continue;
        }
        best_cost[current_pos] = current_cost;

        [
            current_pos as isize + 1,
            current_pos as isize - 1,
            current_pos as isize + SIZE as isize,
            current_pos as isize - SIZE as isize,
        ]
        .into_iter()
        .filter(|&x| x >= START as isize && x <= EXIT as isize)
        .for_each(|p| {
            if grid[p as usize] != b'#' {
                stack.push(Reverse((current_cost + 1, p as usize)));
            }
        });

        grid[current_pos] = b'O';

        // println!("------------------");
        // grid.chunks_exact(SIZE).for_each(|c| {
        //     println!("{}", unsafe { str::from_utf8_unchecked(&c) });
        // });
    }

    0
}

pub fn p2(input: &str) -> usize {
    0
}

