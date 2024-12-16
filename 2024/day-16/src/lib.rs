use aoc::prelude::*;
use itertools::Itertools;
use regex::bytes;

const SIDE_LENGTH: usize = 141;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;

const END: u8 = b'E';
const START: u8 = b'S';
const WALL: u8 = b'#';

const FLAG_VISITED: u8 = 0b1000_0000;

pub fn p1(input: &mut str) -> usize {
    let map = unsafe{input.as_bytes_mut()};
    let start = map.iter().position(|&b| b == START).unwrap();
    let end = map.iter().position(|&b| b == END).unwrap();

    let mut stack = vec![(start,Direction::Right,0usize)];
    while let Some((pos, dir, score)) = stack.pop() {
        if pos == end {
            return score;
        }

        let up = pos - LINE_LENGTH;
        if dir.opposite() != Direction::Up && map[up] != WALL && map[up] & FLAG_VISITED == 0{
            let score = score + if dir == Direction::Up {
                1
            } else {
                1001
            };
            stack.push((up, Direction::Up, score));
        }

        let down = pos + LINE_LENGTH;
        if dir.opposite() != Direction::Down && map[down] != WALL && map[down] & FLAG_VISITED == 0 {
            let score = score + if dir == Direction::Down {
                1
            } else {
                1001
            };
            stack.push((down, Direction::Down, score));
        }

        let left = pos - 1;
        if dir.opposite() != Direction::Left && map[left] != WALL && map[left] & FLAG_VISITED == 0 {
            let score = score + if dir == Direction::Left {
                1
            } else {
                1001
            };
            stack.push((left, Direction::Left, score));
        }

        let right = pos + 1;
        if dir.opposite() != Direction::Right && map[right] != WALL && map[right] & FLAG_VISITED == 0 {
            let score = score + if dir == Direction::Right {
                1
            } else {
                1001
            };
            stack.push((right, Direction::Right, score));
        }

        map[pos] |= FLAG_VISITED;
        stack.sort_by_key(|&(pos, dir, score)| -((h(pos, end, dir) + score) as isize));
    }

    0
}

pub fn p2(input: &str) -> usize {
    0
}

fn h(pos: usize, end: usize, dir: Direction) -> usize {
    let (px, py) = (pos % LINE_LENGTH, pos / LINE_LENGTH);
    let (ex, ey) = (end % LINE_LENGTH, end / LINE_LENGTH);
    let dx = px.abs_diff(ex);
    let dy = py.abs_diff(ey);

    let mut score = dx + dy;
    match dir {
        Direction::Up => if dx > 0 {
            score += 1000;
        },
        Direction::Down => score += 2000,
        Direction::Left => score += 2000,
        Direction::Right => if dy > 0 {
            score += 1000
        }
    }

    score
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
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