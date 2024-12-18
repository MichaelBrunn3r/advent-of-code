#![feature(slice_split_once)]
#![feature(const_for)]
#![feature(generic_const_exprs)]

use aoc::{prelude::*, XY};
use const_for::const_for;
use core::str;
use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

const INNER_SIZE: usize = 71;
pub const SIZE: usize = INNER_SIZE + 2;
const START: usize = SIZE + 1;
const EXIT: usize = SIZE * (SIZE - 1) - 2;

pub type Grid = [u8; SIZE * SIZE];
pub static mut GRID: Grid = generate_grid::<SIZE, SIZE>(b'.', b'#');

pub fn parse(input: &str, grid: &mut Grid) -> Vec<XY<usize, usize>> {
    let mut bytes_iter = input.as_bytes().split(|&b| b == b'\n').take(3450).map(|l| {
        let (x, y) = l.split_once(|&b| b == b',').unwrap();
        xy(x.parse_ascii_digits(), y.parse_ascii_digits())
    });

    for _ in 0..1024 {
        let b = bytes_iter.next().unwrap();
        grid[b.x as usize + SIZE + 1 + b.y as usize * SIZE] = b'#';
    }

    bytes_iter.collect_vec()
}

pub fn p1(grid: &Grid) -> usize {
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
    }

    0
}

pub fn p2(bytes: &[XY<usize, usize>], grid: &mut Grid) -> XY<usize, usize> {
    'outer: for i in 0..3450 - 1024 {
        {
            let p = bytes[i];
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

        return bytes[i];
    }

    xy(0, 0)
}

const fn generate_grid<const W: usize, const H: usize>(fill: u8, border: u8) -> [u8; W * H]
where
    [(); W * H]:,
{
    let mut grid = [fill; W * H];
    const_for!(i in 0..W => {
        grid[i] = border; // top
        grid[i * W] = border; // left
        grid[i * W + (W - 1)] = border; // right
    });
    const_for!(i in (W*(H-1))..(W*H) => {
        grid[i] = border;
    });

    grid
}
