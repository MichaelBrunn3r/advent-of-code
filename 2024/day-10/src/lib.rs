#![feature(hash_set_entry)]

use std::{collections::{hash_set::Entry, HashSet}, mem::zeroed};
use aoc::{prelude::*, ConstVec};
use itertools::Itertools;

const SIDE_LENGTH: usize = 40;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;

pub fn p(input: &mut str) -> (usize, usize) {
    let bytes = unsafe{input.as_bytes_mut()};
    let mut sum_scores = 0;
    let mut sum_ratings = 0;

    let mut stack = Vec::with_capacity(8);

    bytes.iter()
        .enumerate()
        .filter(|(_, &b)| b == b'0')
        .for_each(|(start, _)| {
            let mut visited_9 = HashSet::new();
            stack.clear();
            stack.push(start);

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
                            sum_ratings += 1;
                            if let Entry::Vacant(entry) = visited_9.entry(pos) {
                                sum_scores += 1;
                                entry.insert();
                            }
                        } else {
                            stack.push(pos)
                        }
                    });
            }
        });

    (sum_scores, sum_ratings)
}

