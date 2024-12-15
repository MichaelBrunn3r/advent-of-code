use aoc::prelude::*;
use itertools::Itertools;

const SIDE_LENGTH: usize = 50;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;

const WALL: u8 = b'#';
const ROBOT: u8 = b'@';
const BOX: u8 = b'O';
const EMPTY: u8 = b'.';

pub fn p1(input: &mut str) -> usize {
    let (map, moves) = unsafe { input.as_bytes_mut() }.split_at_mut(SIDE_LENGTH * LINE_LENGTH + 1);
    let mut robot = map.iter().position(|&b| b == b'@').unwrap();

    moves[..moves.len() - 1].iter().for_each(|&m| {
        let step = match m {
            b'\n' => return,
            b'<' => -1,
            b'>' => 1,
            b'^' => -(LINE_LENGTH as isize),
            b'v' => LINE_LENGTH as isize,
            _ => unreachable!(),
        };
        move_robot(&mut robot, step, map);

        // println!("{}", map.as_str_unchecked());
    });

    map
        .chunks_exact(LINE_LENGTH)
        .enumerate()
        .map(|(y, line)| {
            line.iter().enumerate().map(|(x, &b)| {
                if b == BOX {
                    x + y*100 
                } else {
                    0
                }
            }).sum::<usize>()
        }).sum()
}

pub fn p2(input: &str) -> usize {
    0
}

fn move_robot(robot: &mut usize, step: isize, map: &mut [u8]) {
    let next = ((*robot as isize) + step) as usize;
    if let Some(unblocked_empty) = find_unblocked_empty(*robot, step as isize, &map) {
        map[*robot] = EMPTY;
        map[next] = ROBOT;
        *robot = next;

        if *robot != unblocked_empty {
            map[unblocked_empty] = BOX;
        }
    }
}

fn find_unblocked_empty(mut pos: usize, step: isize, map: &[u8]) -> Option<usize> {
    while map[pos] != WALL {
        if map[pos] == EMPTY {
            return Some(pos);
        }
        pos = (pos as isize + step) as usize;
    }
    None
}
