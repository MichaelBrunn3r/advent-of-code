use aoc::prelude::*;
use itertools::Itertools;

pub fn p1(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut checksum = 0;
    let mut l = 0;
    let mut r = bytes.len()-2;
    let mut block_idx = 0;

    let mut r_file_remaining_size = bytes[r] - b'0';
    let mut r_file_id = r >> 1;
    r -= 1;

    'out: while r > l {
        let l_file_size = bytes[l] - b'0';
        let l_file_id = l >> 1;
        l += 1;
        for _ in 0..l_file_size {
            checksum += block_idx * l_file_id;
            block_idx += 1;
        }

        let l_free = bytes[l] - b'0';
        l += 1;
        
        for _ in 0..l_free {
            if r_file_remaining_size == 0 {
                r -= 1;

                if r < l {
                    break 'out;
                }

                r_file_remaining_size = bytes[r] - b'0';
                r_file_id = r >> 1;
                r -= 1;
            }

            checksum += block_idx * r_file_id;
            r_file_remaining_size -= 1;
            block_idx += 1;
        }
    }

    while r_file_remaining_size > 0 {
        checksum += block_idx * r_file_id;
        r_file_remaining_size -= 1;
        block_idx += 1;
    }

    checksum
}

pub fn p2(input: &str) -> usize {
    0
}
