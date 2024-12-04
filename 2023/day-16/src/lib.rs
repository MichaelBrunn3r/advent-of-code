use itertools::Itertools;

/** '.' */
const EMPTY: u8 = b'.';
/** '/' */
const MIRROR_R: u8 = b'/';
/** '\\' */
const MIRROR_L: u8 = b'\\';
/** '|' */
const SPLIT_V: u8 = b'|';
/** '-' */
const SPLIT_H: u8 = b'-';
const WALL: u8 = b'?';

// 11043 too high
pub fn p1(input: &str) -> usize {
    let size = input.find('\n').unwrap();
    let mut tiles = input
        .as_bytes()
        .chunks_exact(size + 1)
        .map(|row| {
            let mut v = row[0..size].to_vec();
            v.insert(0, WALL);
            v.push(WALL);
            v
        })
        .collect_vec();

    tiles.insert(0, vec![WALL; size + 2]);
    tiles.push(vec![WALL; size + 2]);

    let mut energized = vec![vec![false; size + 2]; size + 2];
    let mut unexplored_dirs = vec![vec![0b1111; size + 2]; size + 2];
    let mut stack = vec![(Direction::Right, (1usize, 0usize))];

    while let Some((dir, mut pos)) = stack.pop() {
        let step = match dir {
            Direction::Up => (-1isize, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1isize),
            Direction::Right => (0, 1),
        };

        loop {
            pos = (
                ((pos.0 as isize) + step.0) as usize,
                ((pos.1 as isize) + step.1) as usize,
            );

            if tiles[pos.0][pos.1] == WALL {
                break;
            }

            if unexplored_dirs[pos.0][pos.1] & (dir as u8) == 0 {
                continue;
            }

            let c = tiles[pos.0][pos.1];

            energized[pos.0][pos.1] = true;
            match c {
                SPLIT_V => {
                    match dir {
                        Direction::Left | Direction::Right => {
                            stack.push((Direction::Up, pos));
                            stack.push((Direction::Down, pos));
                            unexplored_dirs[pos.0][pos.1] = 0;
                            break;
                        }
                        Direction::Up | Direction::Down => {}
                    }
                    unexplored_dirs[pos.0][pos.1] = 0;
                }
                SPLIT_H => {
                    match dir {
                        Direction::Up | Direction::Down => {
                            stack.push((Direction::Left, pos));
                            stack.push((Direction::Right, pos));
                            unexplored_dirs[pos.0][pos.1] = 0;
                            break;
                        }
                        Direction::Left | Direction::Right => {}
                    }
                    unexplored_dirs[pos.0][pos.1] = 0;
                }
                MIRROR_L => {
                    match dir {
                        Direction::Up => stack.push((Direction::Left, pos)),
                        Direction::Down => stack.push((Direction::Right, pos)),
                        Direction::Left => stack.push((Direction::Up, pos)),
                        Direction::Right => stack.push((Direction::Down, pos)),
                    }
                    unexplored_dirs[pos.0][pos.1] &= !(dir as u8);
                    break;
                }
                MIRROR_R => {
                    match dir {
                        Direction::Up => stack.push((Direction::Right, pos)),
                        Direction::Down => stack.push((Direction::Left, pos)),
                        Direction::Left => stack.push((Direction::Down, pos)),
                        Direction::Right => stack.push((Direction::Up, pos)),
                    }
                    unexplored_dirs[pos.0][pos.1] &= !(dir as u8);
                    break;
                }
                _ => {}
            }
        }
    }

    // energized.iter().for_each(|row| {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|&b| if b { '#' } else { '.' })
    //             .collect::<String>()
    //     );
    // });

    energized
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum()
}

pub fn p2(input: &str) -> usize {
    0
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}
