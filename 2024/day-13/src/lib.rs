use core::slice;

use aoc::{prelude::*, XY};
use itertools::Itertools;

const NUM_MACHINES: usize = 320;
type Machines = [(XY<i64, i64>, XY<i64, i64>, XY<i64, i64>); NUM_MACHINES];

pub fn parse(input: &str) -> Machines {
    let mut crs = input.as_ptr();
    let mut machines: Machines = unsafe{std::mem::zeroed()};
    for i in 0..NUM_MACHINES {
        crs.skip("Button A: X+".len());
        let a = xy(
            crs.parse_uint::<i64, 2>(),
            crs.skip(4).parse_uint::<i64, 2>(),
        );
        crs.skip("\nButton B: X+".len());

        let b = xy(
            crs.parse_uint::<i64, 2>(),
            crs.skip(4).parse_uint::<i64, 2>(),
        );
        crs.skip("\nPrize: X=".len());

        // abs. freq. digits(X): 3->12, 4->123, 5->95
        let digits_x = if unsafe{*crs.add(4)} == b',' {
            4
        } else if unsafe{*crs.add(5)} == b',' {
            5
        } else {
            3
        };
        let prize_x = crs.parse_uint_n_digits::<i64>(digits_x);
        crs.skip(", Y=".len());

        // abs. freq. digits(Y): 3->8, 4->213, 5->94
        let digits_y = if unsafe{*crs.add(5)} == b'\n' && unsafe{*crs.add(6)} == b'\n' {
            5
        } else if unsafe{*crs.add(4)} == b'\n' && unsafe{*crs.add(5)} == b'\n' {
            4
        } else {
            3
        };
        let prize_y = crs.parse_uint_n_digits::<i64>(digits_y);
        crs.skip(2);
        machines[i] = (a, b, xy(prize_x, prize_y));
    }
    machines
}

pub fn p1(machines: &Machines) -> usize {
    machines.iter()
        .map(|(a, b, prize)| {
            let start_a = {
                let count_a = (prize.y * b.x - prize.x * b.y) / (b.x * a.y - a.x * b.y);
                let count_b = (prize.y - count_a * a.y) / b.y;
    
                if count_a * a.x + count_b * b.x != prize.x || count_a < 1 || count_b < 1 {
                    0
                } else {
                    (3 * count_a + count_b) as usize
                }
            };

            let start_b = {
                let count_b = (prize.y * a.x - prize.x * a.y) / (a.x * b.y - b.x * a.y);
                let count_a = (prize.y - count_b * b.y) / a.y;

                if count_a * a.x + count_b * b.x != prize.x || count_a < 1 || count_b < 1 {
                    0
                } else {
                    (3 * count_a + count_b) as usize
                }
            };

            start_a.min(start_b)
        })
        .sum()
}

pub fn p2(machines: &Machines) -> usize {
    machines.iter()
        .map(|(a, b, prize)| {
            let prize = xy(prize.x + 10000000000000, prize.y + 10000000000000);

            let start_a = {
                let count_a = (prize.y * b.x - prize.x * b.y) / (b.x * a.y - a.x * b.y);
                let count_b = (prize.y - count_a * a.y) / b.y;
    
                if count_a * a.x + count_b * b.x != prize.x || count_a < 1 || count_b < 1 {
                    0
                } else {
                    (3 * count_a + count_b) as usize
                }
            };

            let start_b = {
                let count_b = (prize.y * a.x - prize.x * a.y) / (a.x * b.y - b.x * a.y);
                let count_a = (prize.y - count_b * b.y) / a.y;

                if count_a * a.x + count_b * b.x != prize.x || count_a < 1 || count_b < 1 {
                    0
                } else {
                    (3 * count_a + count_b) as usize
                }
            };

            start_a.min(start_b)
        })
        .sum()
}

