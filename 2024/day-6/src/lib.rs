use aoc::prelude::*;
use itertools::Itertools;

const LINE_LENGTH: usize = 131;
const OBSTACLE: u8 = b'#';

pub fn p1(mut input: &mut str) -> usize {
    let bytes = unsafe{input.as_bytes_mut()};

    let start = bytes.iter().position(|&c| c == b'^').unwrap();

    let mut x = (start % LINE_LENGTH) as i32;
    let mut y = (start / LINE_LENGTH) as i32;
    let mut dir = Direction::UP;
    let (mut vx, mut vy) = dir.velocity();

    loop {
        bytes[((y * (LINE_LENGTH as i32)) + x) as usize] = b'X';

        let x_ahead = x + vx;
        let y_ahead = y + vy;

        let ahead = bytes[((y_ahead * (LINE_LENGTH as i32)) + x_ahead) as usize];

        match ahead {
            OBSTACLE => {
                dir = dir.turn_clockwise();
                (vx, vy) = dir.velocity();
            },
            b'\n' => {
                break;
            },
            _ => {
                x = x_ahead;
                y = y_ahead;
            }
        }
    }

    let r = bytes.iter().filter(|&c| *c == b'X').count();
    // println!("{}", input);
    r
}

pub fn p2(input: &str) -> usize {
    0
}

#[derive(Debug)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

const VELOCITY_LUT: [(i32,i32);4] = [(0,-1), (1,0), (0,1), (-1,0)];
const ASCII_LUT: [u8;4] = [b'^', b'>', b'v', b'<'];

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
