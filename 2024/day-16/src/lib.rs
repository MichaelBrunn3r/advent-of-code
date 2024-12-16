use aoc::{prelude::*, XY};
use std::collections::BinaryHeap;

const SIDE_LENGTH: u16 = 141;
const LINE_LENGTH: u16 = SIDE_LENGTH + 1;

const WALL: u8 = b'#';
const FLAG_VISITED: u8 = 0b1000_0000;

const END: u16 = 2 * LINE_LENGTH - 3;
const START: u16 = (SIDE_LENGTH - 2) * LINE_LENGTH + 1;
const POS_END: XY<u16, u16> = xy(END % LINE_LENGTH, END / LINE_LENGTH);

pub fn p1(input: &mut str) -> usize {
    let map = unsafe { input.as_bytes_mut() };

    let mut queue = BinaryHeap::with_capacity(128);
    queue.push(Node(START, Direction::Right, 0u32));
    while let Some(Node(pos, current_dir, current_score)) = queue.pop() {
        if pos == END {
            return current_score as usize;
        }

        current_dir
            .offsets_orthogonal()
            .iter()
            .for_each(|&(offset, dir)| {
                let pos = (pos as isize + offset) as u16;
                if map[pos as usize] != WALL && map[pos as usize] & FLAG_VISITED == 0 {
                    let score = current_score + 1001;
                    queue.push(Node(pos, dir, score));
                }
            });

        {
            let pos: u16 = (pos as isize + current_dir.offset()) as u16;
            if map[pos as usize] != WALL && map[pos as usize] & FLAG_VISITED == 0 {
                let score = current_score + 1;
                queue.push(Node(pos, current_dir, score));
            }
        }

        map[pos as usize] |= FLAG_VISITED;
    }

    0
}

pub fn p(input: &str) -> (usize, usize) {
    (0, 0)
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn offsets_orthogonal(&self) -> &[(isize, Direction); 2] {
        const SIDES: [[(isize, Direction); 2]; 4] = [
            [
                (Direction::Left.offset(), Direction::Left),
                (Direction::Right.offset(), Direction::Right),
            ],
            [
                (Direction::Left.offset(), Direction::Left),
                (Direction::Right.offset(), Direction::Right),
            ],
            [
                (Direction::Up.offset(), Direction::Up),
                (Direction::Down.offset(), Direction::Down),
            ],
            [
                (Direction::Up.offset(), Direction::Up),
                (Direction::Down.offset(), Direction::Down),
            ],
        ];
        &SIDES[*self as usize]
    }

    const fn offset(&self) -> isize {
        const CONTINUE: [isize; 4] = [-(LINE_LENGTH as isize), LINE_LENGTH as isize, -1, 1];
        CONTINUE[*self as usize]
    }
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
        other.2.cmp(&self.2)
    }
}
