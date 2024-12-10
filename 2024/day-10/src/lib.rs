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
                    .for_each(|adjacent| {
                        let char = unsafe{bytes.as_ptr().offset(adjacent as isize).read()}; // see NOTE 1
                        if char != char_center + 1 {
                            return;
                        }
                        let adjacent = adjacent as usize;

                        if char == b'9' {
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

// NOTE 1: We are not checking bounds here. Reading left/right at the edge of the input
//       will result in reading a b'\n'. But reading left/right at the top-left/bottom-right corner
//       and up/down at the top/bottom DOES lead to out-of-bounds reads.
//       This has not caused any issues yet, but idealy I should add an extra line to the top&bottom
//       of the input to ensure no out-of-bounds reads can occur.