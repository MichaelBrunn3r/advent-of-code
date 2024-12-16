use std::collections::HashSet;

use aoc::prelude::*;
use itertools::Itertools;
use regex::bytes;

const SIDE_LENGTH: usize = 141;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;

const END: u8 = b'E';
const START: u8 = b'S';
const WALL: u8 = b'#';

const FLAG_VISITED: u8 = 0b1000_0000;

pub fn p(input: &str) -> (usize, usize) {
    let map = input.as_bytes();
    let start = map.iter().position(|&b| b == START).unwrap() as u16;
    let end = map.iter().position(|&b| b == END).unwrap() as u16;

    let mut best_paths = Vec::new();
    let mut best_score = 0;

    let mut best_score_at = vec![u32::MAX; map.len()];

    let mut path = HashSet::new();
    path.insert(start);
    let mut stack = vec![(start,Direction::Right,0u32,path)];
    while let Some((pos, dir, score, path)) = stack.pop() {        
        if best_score_at[pos as usize] < u32::MAX && score > best_score_at[pos as usize] + 1000 {
            continue;
        }
        best_score_at[pos as usize] = score;

        if !best_paths.is_empty() && score + h(pos, end, dir) > best_score {
            continue;
        }

        if pos == end {
            best_paths.push(path);
            best_score = score;
            continue;
        }

        let up = pos - LINE_LENGTH as u16;
        if dir.opposite() != Direction::Up && map[up as usize] != WALL && !path.contains(&up){
            let score = score + if dir == Direction::Up {
                1
            } else {
                1001
            };
            let mut path = path.clone();
            path.insert(up);
            stack.push((up, Direction::Up, score, path));
        }

        let down = pos + LINE_LENGTH as u16;
        if dir.opposite() != Direction::Down && map[down as usize] != WALL && !path.contains(&down) {
            let score = score + if dir == Direction::Down {
                1
            } else {
                1001
            };
            let mut path = path.clone();
            path.insert(down);
            stack.push((down, Direction::Down, score, path));
        }

        let left = pos - 1;
        if dir.opposite() != Direction::Left && map[left as usize] != WALL && !path.contains(&left) {
            let score = score + if dir == Direction::Left {
                1
            } else {
                1001
            };
            let mut path = path.clone();
            path.insert(left);
            stack.push((left, Direction::Left, score, path));
        }

        let right = pos + 1;
        if dir.opposite() != Direction::Right && map[right as usize] != WALL && !path.contains(&right) {
            let score = score + if dir == Direction::Right {
                1
            } else {
                1001
            };
            let mut path = path.clone();
            path.insert(right);
            stack.push((right, Direction::Right, score, path));
        }

        stack.sort_by_key(|&(pos, dir, score, _)| -((h(pos, end, dir) + score) as isize));
    }
    
    let best_sit_spots = best_paths.into_iter().fold(HashSet::<u16>::new(), |mut acc,path| {acc.extend(&path); acc});

    (best_score as usize, best_sit_spots.len())
}


fn h(pos: u16, end: u16, dir: Direction) -> u32 {
    let (px, py) = (pos % LINE_LENGTH as u16, pos / LINE_LENGTH as u16);
    let (ex, ey) = (end % LINE_LENGTH as u16, end / LINE_LENGTH as u16);
    let dx = px.abs_diff(ex);
    let dy = py.abs_diff(ey);

    let mut score = (dx + dy) as u32;
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