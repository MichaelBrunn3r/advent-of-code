#![feature(slice_split_once)]
#![feature(const_for)]
#![feature(generic_const_exprs)]

use aoc::{prelude::*, XY};
use const_for::const_for;
use core::str;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

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

pub fn p(bytes: &[XY<usize, usize>], grid: &mut Grid) -> (usize, XY<usize, usize>) {
    let mut stack = VecDeque::with_capacity(290);
    let mut path = vec![START];
    let mut prev = [usize::MAX; SIZE * SIZE];

    let best_cost = find_best_cost(&mut stack, &mut path,&mut prev, &grid).unwrap();

    for i in 0..3450 - 1024 {
        let b = bytes[i];
        let b = input_to_grid(b.x as usize + b.y as usize * SIZE);
        grid[b] = b'#';

        if let Some(_) = path.iter().position(|&p| p == b) {
            path.truncate(1);
            if let None = find_best_cost(&mut stack, &mut path, &mut prev, &grid) {
                return (best_cost, bytes[i]);
            }
        }
    }

    (best_cost, xy(0, 0))
}

fn find_best_cost(stack: &mut VecDeque<(usize, usize)>, path: &mut Vec<usize>, prev: &mut [usize], grid: &Grid) -> Option<usize> {
    let mut visited = [false; SIZE * SIZE];
    stack.clear();
    let start = path[path.len()-1];
    stack.push_back((0usize, start));

    while let Some((current_cost, mut current_pos)) = stack.pop_front() {
        if current_pos == EXIT {
            path.push(current_pos);
            loop {
                current_pos = prev[current_pos];
                if current_pos == start {
                    break;
                }
                path.push(current_pos);
            }
            return Some(current_cost);
        }

        if visited[current_pos] {
            continue;
        }
        visited[current_pos] = true;

        [
            current_pos as isize + 1,
            current_pos as isize + SIZE as isize,
            current_pos as isize - 1,
            current_pos as isize - SIZE as isize,
        ]
        .into_iter()
        .filter(|&x| {
            x >= START as isize
                && x <= EXIT as isize
                && !visited[x as usize]
                && grid[x as usize] != b'#'
        })
        .for_each(|p| {
            stack.push_back((current_cost + 1, p as usize));
            prev[p as usize] = current_pos;
        });
    }

    None
}

fn input_to_grid(pos: usize) -> usize {
    pos + SIZE + 1
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
