use aoc::prelude::*;
use itertools::Itertools;

const NUM_REPORTS: usize = 1000;

pub fn part_1(input: &str) -> usize {
    let mut crs = input.as_ptr();

    (0..NUM_REPORTS).into_iter()
        .map(|_| {
            let mut level_pairs = LevelIterator::new(&mut crs).tuple_windows();
            
            let first: (u8, u8) = level_pairs.next().unwrap();
            let diff = first.1 as i32 - first.0 as i32;

            if diff > 0 {
                diff <= 3 && level_pairs.all(|(a, b)| {
                    a < b && b - a <= 3
                })
            } else if diff < 0 {
                diff >= -3 && level_pairs.all(|(a, b)| {
                    a > b && a - b <= 3
                })
            } else {
                false
            }
        })
        .fold(0, |acc, safe| acc + safe as usize)
}

pub fn part_2(input: &str) -> usize {
    let mut crs = input.as_ptr();

    (0..NUM_REPORTS).into_iter()
        .map(|_| {
            let levels = LevelIterator::new(&mut crs).collect_vec();

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

struct LevelIterator<'a> {
    crs: &'a mut *const u8,
    last_separator: u8
}

impl<'a> LevelIterator<'a> {
    fn new(crs: &'a mut *const u8) -> Self {
        LevelIterator {
            crs,
            last_separator: 0
        }
    }

    fn flush(&mut self) {
        if self.last_separator == b'\n' {
            return;
        }

        while self.crs.take() != b'\n' {}
    }
}

impl<'a> Drop for LevelIterator<'a> {
    fn drop(&mut self) {
        self.flush();
    }
}

impl<'a> Iterator for LevelIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.last_separator == b'\n' {
            return None;
        }

        let mut num = self.crs.take() - b'0';    
        if self.crs.peek() >= b'0' {
            num *= 10;
            num += self.crs.take() - b'0';
        }

        self.last_separator = self.crs.take();

        return Some(num)
    }
}