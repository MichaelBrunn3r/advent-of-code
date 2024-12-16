use std::collections::{BinaryHeap, HashSet};

use aoc::prelude::*;
use itertools::Itertools;
use regex::bytes;

const SIDE_LENGTH: usize = 141;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;

const WALL: u8 = b'#';
const FLAG_VISITED: u8 = 0b1000_0000;

const END: usize = 2 * LINE_LENGTH - 3;
const START: usize = (SIDE_LENGTH - 2) * LINE_LENGTH + 1;

pub fn p1(input: &mut str) -> usize {
    let map = unsafe { input.as_bytes_mut() };

    let mut queue = BinaryHeap::new();
    queue.push(Node(START as u16, Direction::Right, 0u32));
    while let Some(Node(pos, current_dir, score)) = queue.pop() {
        if pos == END as u16 {
            return score as usize;
        }

        [
            (pos - LINE_LENGTH as u16, Direction::Up),
            (pos + LINE_LENGTH as u16, Direction::Down),
            (pos + 1, Direction::Right),
            (pos - 1, Direction::Right),
        ]
        .into_iter()
        .for_each(|(pos, dir)| {
            if current_dir.opposite() != dir
                && map[pos as usize] != WALL
                && map[pos as usize] & FLAG_VISITED == 0
            {
                let score = score + if current_dir == dir { 1 } else { 1001 };
                queue.push(Node(pos, dir, score));
            }
        });

        map[pos as usize] |= FLAG_VISITED;
    }

    0
}

pub fn p(input: &str) -> (usize, usize) {
    (0, 0)
}

#[derive(Debug, PartialEq, Eq)]
struct Node(u16, Direction, u32);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-(h(self.0, self.1) as isize + self.2 as isize)).cmp(&-(h(other.0, other.1) as isize + other.2 as isize))
    }
}


fn h(pos: u16, dir: Direction) -> u32 {
    let (px, py) = (pos % LINE_LENGTH as u16, pos / LINE_LENGTH as u16);
    let (ex, ey) = (END as u16 % LINE_LENGTH as u16, END as u16 / LINE_LENGTH as u16);
    let dx = px.abs_diff(ex);
    let dy = py.abs_diff(ey);

    let mut score = (dx + dy) as u32;
    match dir {
        Direction::Up => {
            if dx > 0 {
                score += 1000;
            }
        }
        Direction::Down => score += 2000,
        Direction::Left => score += 2000,
        Direction::Right => {
            if dy > 0 {
                score += 1000
            }
        }
    }

    score
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
