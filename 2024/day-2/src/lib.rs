use aoc::prelude::*;
use itertools::Itertools;

const NUM_REPORTS: usize = 1000;

pub fn part_1(input: &str) -> usize {
    let mut num_safe = 0;

    let mut crs = input.as_ptr();
    'reports: for _ in 0..NUM_REPORTS {
        let first = parse_level(&mut crs);
        crs.take();
        let mut prev = parse_level(&mut crs);
        crs.take();

        let increasing = first < prev;
        if first == prev || first.abs_diff(prev) > 3 {
            skip_to_next_line(&mut crs);
            continue 'reports;
        }

        loop {
            let level = parse_level(&mut crs);

            if level == prev
                || level.abs_diff(prev) > 3
                || (increasing && prev > level)
                || (!increasing && prev < level)
            {
                skip_to_next_line(&mut crs);
                continue 'reports;
            }
            
            let separator = crs.take();
            prev = level;

            if separator == b'\n' {
                break;
            }
        }

        num_safe += 1;
    }

    num_safe
}

fn parse_level(crs: &mut *const u8) -> u8 {
    let mut num = crs.take() - b'0';    
    if crs.peek() >= b'0' {
        num *= 10;
        num += crs.take() - b'0';
    }

    num
}

fn skip_to_next_line(crs: &mut *const u8) {
    while crs.take() != b'\n' {}
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
