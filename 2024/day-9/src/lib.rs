#![feature(ascii_char)]

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
    let bytes = input.as_bytes();

    let mut checksum = 0;

    let mut files = Vec::new();
    let mut free = Vec::new();

    let mut i = 0;
    let mut block_idx = 0usize;
    while i < bytes.len()-2 {
        let file_size = (bytes[i] - b'0') as usize; 
        files.push((i >> 1, file_size, block_idx));
        block_idx += file_size as usize;
        i += 1;

        let free_size = (bytes[i] - b'0') as usize;
        free.push((free_size, block_idx));
        block_idx += free_size as usize;
        i += 1;
    }
    files.push(((bytes.len()-2) / 2, (bytes[bytes.len()-2] - b'0') as usize, block_idx));    

    for (file_id, file_size, file_block_idx) in files.into_iter().rev() {
        let start_block_idx = if let Some(free_pos) = free.iter().position(|&(size, block_idx)| size >= file_size && block_idx < file_block_idx) {
            let (_, free_block_idx) = free[free_pos];
            free[free_pos].0 -= file_size;
            free[free_pos].1 += file_size;
            free_block_idx
        } else {
            file_block_idx
        };

        checksum += file_id * (start_block_idx..start_block_idx+file_size).sum::<usize>();
        free.pop();
    }

    checksum
}
