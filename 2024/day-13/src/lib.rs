use aoc::{prelude::*, XY};
use itertools::Itertools;

type Machines = Vec<(XY<i64, i64>, XY<i64, i64>, XY<i64, i64>)>;


pub fn parse(input: &str) -> Machines {
    let mut bytes = input.as_bytes();
    let mut machines = Vec::new();
    loop {
        bytes = &bytes["Button A: X+".len()..];
        let a = xy(
            bytes[..2].parse_ascii_digits() as i64,
            bytes[6..8].parse_ascii_digits() as i64,
        );
        bytes = &bytes["??, Y+??\nButton B: X+".len()..];
        let b = xy(
            bytes[..2].parse_ascii_digits() as i64,
            bytes[6..8].parse_ascii_digits() as i64,
        );
        bytes = &bytes["??, Y+??\nPrize: X=".len()..];

        // abs. freq. digits(X): 3->12, 4->123, 5->95
        let digits_x = if bytes[4] == b',' {
            4
        } else if bytes[5] == b',' {
            5
        } else {
            3
        };
        let prize_x = bytes.parse_n_ascii_digits(digits_x) as i64;
        bytes = &bytes[digits_x + ", Y=".len()..];

        // abs. freq. digits(Y): 3->8, 4->213, 5->94
        let digits_y = if bytes[3] == b'\n' {
            3
        } else if bytes[4] == b'\n' {
            4
        } else {
            5
        };
        let prize_y = bytes.parse_n_ascii_digits(digits_y) as i64;
        bytes = &bytes[digits_y..];

        bytes = &bytes[1..];

        machines.push((a, b, xy(prize_x, prize_y)));

        if bytes.is_empty() || bytes[0] != b'\n' {
            break;
        }

        bytes = &bytes[1..];
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

