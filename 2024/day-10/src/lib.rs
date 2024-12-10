#![feature(hash_set_entry)]

use std::collections::{hash_set::Entry, HashSet};
use aoc::prelude::*;
use itertools::Itertools;

const SIDE_LENGTH: usize = 40;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;
const NUM_BYTES: usize = LINE_LENGTH * SIDE_LENGTH;

pub fn p1(input: &str) -> usize {
    let bytes = input.as_bytes();
    bytes.iter()
        .enumerate()
        .filter(|(_, &b)| b == b'0')
        .map(|(start, _)| {
            let mut score = 0;
            let mut visited_9 = HashSet::new();
            let mut stack = vec![start];

            while let Some(pos_center) = stack.pop() {
                let pos_next = [pos_center as i32 - LINE_LENGTH as i32, (pos_center + LINE_LENGTH) as i32, pos_center as i32 - 1i32, (pos_center + 1) as i32];
                pos_next.into_iter()
                    .filter(|&pos| pos >= 0 && pos < bytes.len() as i32)
                    .for_each(|pos| {
                        let pos = pos as usize;
                        if bytes[pos] != bytes[pos_center] + 1 {
                            return;
                        }

                        if bytes[pos] == b'9' {
                            if let Entry::Vacant(entry) = visited_9.entry(pos) {
                                score += 1;
                                entry.insert();
                            }
                        } else {
                            stack.push(pos)
                        }
                    });
            }
            score
        })
        .sum()
}

pub fn p2(input: &str) -> usize {
    0
}
