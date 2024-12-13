use aoc::prelude::*;
use itertools::Itertools;

pub fn p1(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .tuples()
        .map(|(la, lb, lprize, _)| {
            let a = XY::new(
                la[12..14].parse_ascii_digits() as i64,
                la[18..20].parse_ascii_digits() as i64,
            );
            let b = XY::new(
                lb[12..14].parse_ascii_digits() as i64,
                lb[18..20].parse_ascii_digits() as i64,
            );

            let pos_comma = lprize.iter().position(|&b| b == b',').unwrap();
            let mut prize = XY::new(
                lprize[9..pos_comma].parse_ascii_digits() as i64,
                lprize[pos_comma + 4..].parse_ascii_digits() as i64,
            );

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

pub fn p2(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .tuples()
        .map(|(la, lb, lprize, _)| {
            let a = XY::new(
                la[12..14].parse_ascii_digits() as i64,
                la[18..20].parse_ascii_digits() as i64,
            );
            let b = XY::new(
                lb[12..14].parse_ascii_digits() as i64,
                lb[18..20].parse_ascii_digits() as i64,
            );

            let pos_comma = lprize.iter().position(|&b| b == b',').unwrap();
            let mut prize = XY::new(
                lprize[9..pos_comma].parse_ascii_digits() as i64,
                lprize[pos_comma + 4..].parse_ascii_digits() as i64,
            );
            prize.x += 10000000000000;
            prize.y += 10000000000000;

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

struct XY<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> XY<X, Y> {
    fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }
}
