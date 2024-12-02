use std::io::Read;

use aoc::prelude::*;
use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            LevelIterator{crs: line.as_ptr()}
                .tuple_windows()
                .map(|(a, b)| a as i32 - b as i32)
                .tuple_windows()
                .all(|(a, b)| {
                    return (a >= 1 && a <= 3 && b >= 1 && b <= 3) 
                        || (a <= -1 && a >= -3 && b <= -1 && b >= -3);
                })
        })
        .filter(|line| *line)
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let levels = LevelIterator{crs: line.as_ptr()}.collect_vec();

            for skipi in 0..levels.len() {
                if levels[..skipi].iter().chain(levels[skipi+1..].iter())
                    .tuple_windows()
                    .map(|(a, b)| *a as i32 - *b as i32)
                    .tuple_windows()
                    .all(|(a, b)| {
                        return (a >= 1 && a <= 3 && b >= 1 && b <= 3) 
                            || (a <= -1 && a >= -3 && b <= -1 && b >= -3);
                    })
                {
                    return true;
                }
            }

            false
        })
        .filter(|line| *line)
        .count()
}

struct LevelIterator {
    crs: *const u8,
}

impl Iterator for LevelIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.crs.peek() == b'\n' {
            return None;
        }

        let mut num = self.crs.take() - b'0';
        if self.crs.peek() >= b'0' {
            num *= 10;
            num += self.crs.take() - b'0';
        }

        if self.crs.peek() == b' ' {
            self.crs.take();
        }

        return Some(num);
    }
}
