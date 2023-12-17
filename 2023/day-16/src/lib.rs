#![allow(dead_code)]

use std::hint::unreachable_unchecked;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const EMPTY: u8 = b'.';
const MIRROR_R: u8 = b'/';
const MIRROR_L: u8 = b'\\';
const SPLIT_V: u8 = b'|';
const SPLIT_H: u8 = b'-';
const WALL: u8 = b'#';

const TOP_ID: usize = 0;
const BOTTOM_ID: usize = 1;
const LEFT_ID: usize = 2;
const RIGHT_ID: usize = 3;
const START_ID: usize = 4;

pub fn part_1(input: &str) -> usize {
    let size = input.find('\n').unwrap();
    let mut net = create_network(input.as_bytes(), size);

    let start = net[START_ID];
    let mut num_energized_tiles = start.col - 1;
    let mut todo = vec![(Direction::Right, START_ID)];
    println!("energized: {:?}", num_energized_tiles);

    while !todo.is_empty() {
        let (dir, node_id) = todo.pop().unwrap();
        let node = &net[node_id];
        let visited = node.visited;

        if node.allowed_directions == 0 {
            continue;
        }

        println!(
            "{}: {:?}, ({}, {}), {:?}",
            node_id, node.kind, node.row, node.col, dir
        );

        match node.kind {
            NodeKind::SplitV => {
                if dir.is_vertical() {
                    let next_id = match dir {
                        Direction::Up => node.up,
                        Direction::Down => node.down,
                        _ => unsafe { unreachable_unchecked() },
                    };

                    let next = &net[next_id];
                    num_energized_tiles += node.row.abs_diff(next.row) - 1;

                    todo.push((dir, next_id));
                } else {
                    let above = &net[node.up];
                    num_energized_tiles += node.row.abs_diff(above.row) - 1;
                    todo.push((Direction::Up, node.up));

                    let below = &net[node.down];
                    num_energized_tiles += node.row.abs_diff(below.row) - 1;
                    todo.push((Direction::Down, node.down));
                }
                net[node_id].allowed_directions = 0u8;
            }
            NodeKind::SplitH => {
                if dir.is_horizontal() {
                    let next_id = match dir {
                        Direction::Left => node.left,
                        Direction::Right => node.right,
                        _ => unsafe { unreachable_unchecked() },
                    };

                    let next = &net[next_id];
                    num_energized_tiles += node.col.abs_diff(next.col) - 1;

                    todo.push((dir, next_id));
                } else {
                    let left = &net[node.left];
                    num_energized_tiles += node.col.abs_diff(left.col) - 1;
                    todo.push((Direction::Left, node.left));

                    let right = &net[node.right];
                    num_energized_tiles += node.col.abs_diff(right.col) - 1;
                    todo.push((Direction::Right, node.right));
                }
                net[node_id].allowed_directions = 0u8;
            }
            NodeKind::MirrorR => {
                let (next_id, next_dir) = match dir {
                    Direction::Up => (node.right, Direction::Right),
                    Direction::Down => (node.left, Direction::Left),
                    Direction::Left => (node.down, Direction::Down),
                    Direction::Right => (node.up, Direction::Up),
                };

                let next = &net[next_id];
                num_energized_tiles += match next_dir {
                    Direction::Up => node.row.abs_diff(next.row) - 1,
                    Direction::Down => node.row.abs_diff(next.row) - 1,
                    Direction::Left => node.col.abs_diff(next.col) - 1,
                    Direction::Right => node.col.abs_diff(next.col) - 1,
                };
                todo.push((next_dir, next_id));

                net[node_id].allowed_directions &= !(dir as u8);
            }
            NodeKind::MirrorL => {
                let (next_id, next_dir) = match dir {
                    Direction::Up => (node.left, Direction::Left),
                    Direction::Down => (node.right, Direction::Right),
                    Direction::Left => (node.up, Direction::Up),
                    Direction::Right => (node.down, Direction::Down),
                };
                let next = &net[next_id];
                num_energized_tiles += match next_dir {
                    Direction::Up => node.row.abs_diff(next.row) - 1,
                    Direction::Down => node.row.abs_diff(next.row) - 1,
                    Direction::Left => node.col.abs_diff(next.col) - 1,
                    Direction::Right => node.col.abs_diff(next.col) - 1,
                };
                todo.push((next_dir, next_id));

                net[node_id].allowed_directions &= !(dir as u8);
            }
            _ => {}
        }

        if !visited {
            num_energized_tiles += 1;
            net[node_id].visited = true;
        }

        println!("  num_energized_tiles: {}", num_energized_tiles);
    }

    num_energized_tiles
}

pub fn part_2(input: &str) -> usize {
    0
}

fn create_network(input: &[u8], size: usize) -> Vec<Node> {
    let top = Node::wall(0, 0);
    let bottom = Node::wall(size + 1, 0);
    let left = Node::wall(0, 0);
    let right = Node::wall(0, size + 1);
    let mut nodes = vec![top, bottom, left, right];

    let mut above_ids = (0..size + 2).map(|_| TOP_ID).collect_vec();
    for (cells, row) in input.chunks_exact(size + 1).zip(1..size + 1) {
        let mut left = LEFT_ID;

        for (&cell, col) in cells[0..size].iter().zip(1..size + 1) {
            match cell {
                MIRROR_R | MIRROR_L | SPLIT_V | SPLIT_H => {
                    // Create Node
                    let node = Node::new(
                        cell.into(),
                        row,
                        col,
                        left,
                        RIGHT_ID,
                        above_ids[col],
                        BOTTOM_ID,
                    );
                    let node_id = nodes.len();
                    nodes.push(node);

                    // Update left
                    nodes[left].right = node_id;
                    left = node_id;

                    // Update above
                    nodes[above_ids[col]].down = node_id;
                    above_ids[col] = node_id;
                }
                _ => {}
            }
        }
    }

    nodes
}

#[derive(Debug, Copy, Clone)]

struct Node {
    kind: NodeKind,
    row: usize,
    col: usize,
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    allowed_directions: u8,
    visited: bool,
}

impl Node {
    fn new(
        kind: NodeKind,
        row: usize,
        col: usize,
        left: usize,
        right: usize,
        up: usize,
        down: usize,
    ) -> Self {
        Self {
            kind,
            row,
            col,
            left,
            right,
            up,
            down,
            allowed_directions: kind.initialy_allowed_directions(),
            visited: false,
        }
    }

    fn wall(row: usize, col: usize) -> Self {
        Self {
            kind: NodeKind::Wall,
            row,
            col,
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            allowed_directions: NodeKind::Wall.initialy_allowed_directions(),
            visited: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

impl Direction {
    #[inline(always)]
    fn is_horizontal(&self) -> bool {
        (*self as u8 & 0b1100) != 0
    }

    #[inline(always)]
    fn is_vertical(&self) -> bool {
        (*self as u8 & 0b0011) != 0
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum NodeKind {
    /** '/' */
    MirrorR,
    /** '\\' */
    MirrorL,
    /** '-' */
    SplitH,
    /** '|' */
    SplitV,
    Wall,
}

impl NodeKind {
    fn initialy_allowed_directions(&self) -> u8 {
        match self {
            Self::MirrorR => 0b1111,
            Self::MirrorL => 0b1111,
            Self::SplitH => Direction::Up as u8 | Direction::Down as u8,
            Self::SplitV => Direction::Left as u8 | Direction::Right as u8,
            Self::Wall => 0,
        }
    }
}

impl From<u8> for NodeKind {
    fn from(byte: u8) -> Self {
        match byte {
            MIRROR_R => Self::MirrorR,
            MIRROR_L => Self::MirrorL,
            SPLIT_V => Self::SplitV,
            SPLIT_H => Self::SplitH,
            WALL => Self::Wall,
            _ => panic!("Invalid node kind"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:04b}", Direction::Up as u8);
    }
}
