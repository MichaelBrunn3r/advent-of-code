use core::time;
use std::thread;

use aoc::{prelude::*, XY};
use itertools::Itertools;

const NUM_ROBOTS: usize = 500;
const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const PERIOD: usize = WIDTH * HEIGHT;

pub fn p1(input: &str) -> usize {
    let time = 100;
    let mut quadrants = [0, 0, 0, 0];
    let mut crs = input.as_bytes().as_ptr();
    for _ in 0..NUM_ROBOTS {
        let (mut pos, v) = parse_robot(&mut crs);
        // println!("start={:?} {:?}", pos, v);

        let mut x = (pos.x as isize + time * v.x) % WIDTH as isize;
        let mut y = (pos.y as isize + time * v.y) % HEIGHT as isize;
        if x < 0 {
            x = WIDTH as isize + x;
        }
        if y < 0 {
            y = HEIGHT as isize + y;
        }

        pos.x = x as usize;
        pos.y = y as usize;

        if pos.x == WIDTH / 2 || pos.y == HEIGHT / 2 {
            continue;
        }

        quadrants[(pos.y < (HEIGHT / 2)) as usize | (((pos.x < (WIDTH / 2)) as usize) << 1)] += 1;
    }

    quadrants.into_iter().product()
}

pub fn p2(input: &str) -> usize {
    let mut crs = input.as_bytes().as_ptr();
    let mut robots = (0..NUM_ROBOTS).map(|_| parse_robot(&mut crs)).collect_vec();

    for i in 1..PERIOD {
        for (pos, v) in robots.iter_mut() {
            let mut x = (pos.x as isize + v.x) % WIDTH as isize;
            let mut y = (pos.y as isize + v.y) % HEIGHT as isize;
            if x < 0 {
                x = WIDTH as isize + x;
            }
            if y < 0 {
                y = HEIGHT as isize + y;
            }

            pos.x = x as usize;
            pos.y = y as usize;
        }

        let diff: usize = robots
            .iter()
            .map(|r| {
                ((r.0.x as isize - (WIDTH as isize /2)).abs()
                    + (r.0.y as isize - (HEIGHT as isize /2)).abs()) as usize
            })
            .sum::<usize>();

        if diff <= 18_000 {
            return i;
        }
    }

    0
}

fn parse_robot(crs: &mut *const u8) -> (XY<usize, usize>, XY<isize, isize>) {
    crs.skip("p=".len());

    // Abs. freq.: 1->52, 2->441, 3->7
    let x: usize = crs.parse_uint_n_digits(if unsafe { *crs.add(2) } == b',' {
        2
    } else if unsafe { *crs.add(1) } == b',' {
        1
    } else {
        3
    });
    crs.skip(",".len());

    // Abs. freq.: 1->52, 2->441, 3->7
    let y: usize = crs.parse_uint_n_digits(if unsafe { *crs.add(2) } == b' ' {
        2
    } else if unsafe { *crs.add(1) } == b' ' {
        1
    } else {
        3
    });
    crs.skip(" v=".len());

    let neg = unsafe { crs.read() } == b'-';
    if neg {
        crs.skip(",".len());
    }

    // Abs. freq.: 1->41, 2->459
    let mut vx: isize = crs.parse_uint_n_digits(if unsafe { *crs.add(2) } == b',' { 2 } else { 1 });
    if neg {
        vx *= -1;
    }
    crs.skip(",".len());

    let neg = unsafe { crs.read() } == b'-';
    if neg {
        crs.skip(",".len());
    }

    // Abs. freq.: 1->38, 2->462
    let mut vy: isize = crs.parse_uint_n_digits(if unsafe { *crs.add(2) } == b'\n' {
        2
    } else {
        1
    });
    if neg {
        vy *= -1;
    }
    crs.skip("\n".len());

    (xy(x, y), xy(vx, vy))
}
