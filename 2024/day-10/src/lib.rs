#![feature(hash_set_entry)]

use aoc::{prelude::*, ConstVec};
use itertools::Itertools;

const SIDE_LENGTH: usize = 40;
const LINE_LENGTH: usize = SIDE_LENGTH + 1;

pub fn p(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();
    let (mut sum_scores, mut sum_ratings) = (0, 0);
    let mut stack = ConstVec::<u16, 8>::new();

    bytes
        .iter()
        .enumerate()
        .filter(|(_, &b)| b == b'0')
        .for_each(|(idx_start, _)| {
            let mut visited = [0u64; SIDE_LENGTH];
            stack.clear();
            stack.push(idx_start as u16);

            while let Some(idx_center) = stack.pop() {
                let char_center = bytes[idx_center as usize];
                [-1, 1, -(LINE_LENGTH as i32), (LINE_LENGTH as i32)]
                    .into_iter()
                    .map(|offset| idx_center as i32 + offset)
                    .filter(|&pos| pos >= 0 && pos < bytes.len() as i32)
                    .for_each(|adjacent| {
                        let adjacent = adjacent as usize;
                        if bytes[adjacent] != char_center + 1 {
                            return;
                        }

                        if bytes[adjacent] == b'9' {
                            let x = adjacent % LINE_LENGTH;
                            let y = adjacent / LINE_LENGTH;

                            sum_ratings += 1;
                            if visited[y] & 1 << x == 0 {
                                sum_scores += 1;
                                visited[y] |= 1 << x;
                            }
                        } else {
                            stack.push(adjacent as u16)
                        }
                    });
            }
        });

    (sum_scores, sum_ratings)
}
