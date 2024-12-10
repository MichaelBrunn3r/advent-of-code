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
        .for_each(|(idx_start, _)| {
            let mut visited_9 = [0u64; SIDE_LENGTH];
            stack.clear();
            stack.push(idx_start);

            while let Some(idx_center) = stack.pop() {
                let char_center = bytes[idx_center];

                [idx_center as i32 - LINE_LENGTH as i32, (idx_center + LINE_LENGTH) as i32, idx_center as i32 - 1i32, (idx_center + 1) as i32].into_iter()
                    .filter(|&pos| pos >= 0 && pos < bytes.len() as i32)
                    .for_each(|pos| {
                        let pos = pos as usize; 
                        let char = bytes[pos];      

                        if char != char_center + 1 {
                            return;
                        }

                        if char == b'9' {
                            let x = pos % LINE_LENGTH;  
                            let y = pos / LINE_LENGTH;

                            sum_ratings += 1;
                            if visited_9[y] & 1 << x == 0 {
                                sum_scores += 1;
                                visited_9[y] |= 1 << x;
                            }
                        } else {
                            stack.push(pos)
                        }
                    });
            }
        });

    (sum_scores, sum_ratings)
}

