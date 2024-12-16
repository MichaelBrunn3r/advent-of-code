use std::collections::{BinaryHeap};
use aoc::{prelude::*, XY};

const SIDE_LENGTH: u16 = 141;
const LINE_LENGTH: u16 = SIDE_LENGTH + 1;

const WALL: u8 = b'#';
const FLAG_VISITED: u8 = 0b1000_0000;

const END: u16 = 2 * LINE_LENGTH - 3;
const START: u16 = (SIDE_LENGTH - 2) * LINE_LENGTH + 1;
const POS_END: XY<u16,u16> = xy(END % LINE_LENGTH, END / LINE_LENGTH);

pub fn p1(input: &mut str) -> usize {
    let map = unsafe { input.as_bytes_mut() };

    let mut queue = BinaryHeap::with_capacity(128);
    queue.push(Node(START, Direction::Right, 0u32, h(START)));
    while let Some(Node(pos, current_dir, score, _)) = queue.pop() {
        if pos == END {
            return score as usize;
        }

        [
            (pos - LINE_LENGTH, Direction::Up),
            (pos + 1, Direction::Right),
            (pos + LINE_LENGTH, Direction::Down),
            (pos - 1, Direction::Left),
        ]
        .into_iter()
        .for_each(|(pos, dir)| {
            if current_dir.opposite() != dir
                && map[pos as usize] != WALL
                && map[pos as usize] & FLAG_VISITED == 0
            {
                let score = score + if current_dir == dir { 1 } else { 1001 };
                queue.push(Node(pos, dir, score, h(pos) + score));
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
struct Node(u16, Direction, u32, u32);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.3.cmp(&self.3)
    }
}

fn h(pos: u16) -> u32 {
    let (px, py) = (pos % LINE_LENGTH, pos / LINE_LENGTH);
    let dx = px.abs_diff(POS_END.x);
    let dy = py.abs_diff(POS_END.y);
    (dx + dy) as u32
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[repr(u8)]
enum Direction {
    Up=0,
    Down=1,
    Left=2,
    Right=3,
}

impl Direction {
    fn opposite(&self) -> Self {
        const OPPOSITE: [Direction; 4] = [
            Direction::Down,
            Direction::Up,
            Direction::Right,
            Direction::Left
        ];
        OPPOSITE[*self as usize]
    }
}
