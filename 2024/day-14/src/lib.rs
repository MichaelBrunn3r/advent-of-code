use core::time;
use std::thread;

use aoc::{prelude::*, XY};
use itertools::Itertools;

const NUM_ROBOTS: usize = 500;
const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const PERIOD_X: usize = WIDTH;
const PERIOD_Y: usize = HEIGHT;

type Robot = (XY<usize, usize>, XY<isize, isize>);

pub fn p(input: &str) -> (usize, usize) {
    let time = 100;
    let mut quadrants = [0, 0, 0, 0];
    let mut robots: [Robot; NUM_ROBOTS] = unsafe{std::mem::zeroed()};

    let mut crs = input.as_bytes().as_ptr();
    for i in 0..NUM_ROBOTS {
        let (mut pos, v) = parse_robot(&mut crs);
        robots[i] = (pos, v);

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

    let mut min_var = xy(isize::MAX, isize::MAX);
    let mut min_var_t = xy(0, 0);
    for t in 1..PERIOD_X.max(PERIOD_Y)+1 {
        for (p, v) in robots.iter_mut() {
            let mut x = (p.x as isize + v.x) % WIDTH as isize;
            let mut y = (p.y as isize + v.y) % HEIGHT as isize;
            if x < 0 {
                x = WIDTH as isize + x;
            }
            if y < 0 {
                y = HEIGHT as isize + y;
            }
            p.x = x as usize;
            p.y = y as usize;
        }

        let var = variance(&robots);
        if var.x < min_var.x {
            min_var.x = var.x;
            min_var_t.x = t;
        }if var.y < min_var.y {
            min_var.y = var.y;
            min_var_t.y = t;
        }
    }

    let inv_w = 51;
    let t = (min_var_t.x as usize + ((inv_w * (min_var_t.y as isize - min_var_t.x as isize)) % HEIGHT as isize) as usize * WIDTH) as usize;

    (quadrants.into_iter().product(), t)
}

fn variance(robots: &[Robot]) -> XY<isize, isize> {
    let mean_x = robots.iter().map(|(p, _)| p.x).sum::<usize>() / NUM_ROBOTS;
    let mean_y = robots.iter().map(|(p, _)| p.y).sum::<usize>() / NUM_ROBOTS;
    let sum_sq_diff_x = robots.iter().map(|(p,_)| (p.x as isize - mean_x as isize).pow(2) ).sum::<isize>();
    let sum_sq_diff_y = robots.iter().map(|(p,_)| (p.y as isize - mean_y as isize).pow(2) ).sum::<isize>();
    let var_x = sum_sq_diff_x / NUM_ROBOTS as isize;
    let var_y = sum_sq_diff_y / NUM_ROBOTS as isize;

    xy(var_x, var_y)
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
