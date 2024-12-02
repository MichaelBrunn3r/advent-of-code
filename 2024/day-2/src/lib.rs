use aoc::prelude::*;
use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line
                .split(" ")
                .map(|digits| digits.as_bytes().parse_ascii_digits() as i32)
                .tuple_windows()
                .map(|(a, b)| a - b)
                .tuple_windows()
                .all(|(a, b)| {
                    return (a >= 1 && a <= 3 && b >= 1 && b <= 3) 
                        || (a <= -1 && a >= -3 && b <= -1 && b >= -3);
                })
        })
        .filter(|line| *line)
        .count()
}
