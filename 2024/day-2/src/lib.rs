#![feature(iter_collect_into)]

use aoc::{prelude::*};
use itertools::Itertools;

const NUM_REPORTS: usize = 1000;

pub fn p1(input: &str) -> usize {
    let mut crs = input.as_ptr();

    (0..NUM_REPORTS).into_iter()
        .map(|_| is_report_safe(LevelIterator::new(&mut crs).tuple_windows()))
        .fold(0, |acc, safe| acc + safe as usize)
}

pub fn p2(input: &str) -> usize {
    let mut crs = input.as_ptr();
    let mut levels = Vec::with_capacity(10);

    (0..NUM_REPORTS).into_iter()
        .map(|_| {
            levels.clear();
            LevelIterator::new(&mut crs).collect_into(&mut levels);
            (0..levels.len()).into_iter()
                .any(|skip_idx| {
                    let patched_levels = levels[..skip_idx].iter().chain(levels[skip_idx+1..].iter());
                    is_report_safe(patched_levels.map(|x| *x).tuple_windows())
                })
        })
        .filter(|line| *line)
        .count()
}

fn is_report_safe(mut level_pairs: impl Iterator<Item=(u8, u8)>) -> bool {
    let first = level_pairs.next().unwrap();
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