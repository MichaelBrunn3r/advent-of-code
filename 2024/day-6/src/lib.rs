use aoc::prelude::*;
use itertools::Itertools;

const SIDE_LENGTH: usize = 130;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;
const OBSTACLE: u8 = b'#';

// # = 10_0011
// . = 10_1110

pub fn p1(input: &mut str) -> usize {
    let bytes = unsafe{input.as_bytes_mut()};

    let mut visited = [0; SIDE_LENGTH*(SIDE_LENGTH+1)];
    let start = bytes.iter().position(|&c| c == b'^').unwrap();
    bytes[start] = b'.';
    
    let mut x = (start % LINE_LENGTH) as i32;
    let mut y = (start / LINE_LENGTH) as i32;
    let mut dir = Direction::UP;
    let (mut vx, mut vy) = dir.velocity();
    
    loop {
        let x_ahead = x + vx;
        let y_ahead = y + vy;

        let pos_current = (y * (LINE_LENGTH as i32)) + x;
        let pos_ahead = ((y_ahead * (LINE_LENGTH as i32)) + x_ahead) as usize;
        let ahead = bytes[pos_ahead];

        match ahead {
            OBSTACLE => {
                dir = dir.turn_clockwise();
                (vx, vy) = dir.velocity();
            },
            b'\n' => {
                visited[pos_current as usize] |= 1 << dir.as_u8();
                break;
            },
            _ => {
                visited[pos_current as usize] |= 1 << dir.as_u8();
                x = x_ahead;
                y = y_ahead;
            }
        }
    }

    let cycles = (0..bytes.len())
        .into_iter()
        .filter(|&start| {
            let mut x = (start % LINE_LENGTH) as i32;
            let mut y = (start / LINE_LENGTH) as i32;
            let mut dir = Direction::UP;
            let (mut vx, mut vy) = dir.velocity();

            let mut visited = [0; SIDE_LENGTH*LINE_LENGTH];

            loop {
                let x_ahead = x + vx;
                let y_ahead = y + vy;
                let pos_current = ((y * (LINE_LENGTH as i32)) + x) as usize;
                let pos_ahead = ((y_ahead * (LINE_LENGTH as i32)) + x_ahead) as usize;

                if x_ahead < 0 || y_ahead < 0 || pos_current > visited.len() || pos_ahead > bytes.len() {
                    return false;
                }
        
                let ahead = bytes[pos_ahead];

                if visited[pos_current] > 5 {
                    return true;
                }
        
                match ahead {
                    OBSTACLE => {
                        dir = dir.turn_clockwise();
                        (vx, vy) = dir.velocity();
                    },
                    b'\n' => {
                        return false;
                    },
                    _ => {
                        visited[pos_current] += 1;
                        x = x_ahead;
                        y = y_ahead;
                    }
                }
            }
        })
        .count();

    println!("{cycles}");

    let num_visited = visited.iter().filter(|&&v| v > 0).count();


    num_visited
}

pub fn p2(input: &str) -> usize {
    0
}

#[derive(Debug, Clone)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

const VELOCITY_LUT: [(i32,i32);4] = [(0,-1), (1,0), (0,1), (-1,0)];

impl Direction {
    fn velocity(&self) -> (i32,i32) {
        VELOCITY_LUT[unsafe{*std::mem::transmute::<&Direction, &u8>(self)} as usize]
    }

    fn turn_clockwise(&self) -> Self {
        ((self.as_u8() + 1) % 4).into()
    }

    fn as_u8(&self) -> &u8 {
        unsafe{std::mem::transmute::<&Direction, &u8>(self)}
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        unsafe{std::mem::transmute(value)}
    }
}
