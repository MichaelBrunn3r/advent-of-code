#![feature(slice_split_once)]
use aoc::{prelude::*, XY};
use itertools::Itertools;
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
    }

    0
}

// 2976
pub fn p2(input: &str) -> XY<usize, usize> {
    let mut grid = [b'.'; SIZE * SIZE];
    (0..SIZE).for_each(|i| {
        grid[i] = b'#';
        grid[i * SIZE] = b'#';
        grid[i * SIZE] = b'#';
        grid[i * SIZE + (SIZE - 1)] = b'#';
    });
    ((SIZE - 1) * SIZE..SIZE * SIZE).for_each(|i| grid[i] = b'#');

    let bytes = input
    .as_bytes()
    .split(|&b| b == b'\n')
    .take(3450)
    .map(|l| {
        let (x, y) = l.split_once(|&b| b == b',').unwrap();
        xy(x.parse_ascii_digits(), y.parse_ascii_digits())
    }).collect_vec();

    bytes.iter().take(1024).for_each(|p| {
        grid[p.x as usize + SIZE + 1 + p.y as usize * SIZE] = b'#';
    });
    
    'outer: for i in 1024..=3450 {
        // println!("{i}");

        {
            let p = bytes[i-1];
            // println!("{:?}", p);
            grid[p.x as usize + SIZE + 1 + p.y as usize * SIZE] = b'#';
        }

        let mut best_cost = [usize::MAX; SIZE * SIZE];
        let mut stack = BinaryHeap::new();
        stack.push(Reverse((0usize, START)));
        while let Some(Reverse((current_cost, current_pos))) = stack.pop() {
            if current_pos == EXIT {
                continue 'outer;
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
        }

        return bytes[i-1];
    }

    xy(0, 0)
}
