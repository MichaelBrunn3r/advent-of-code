#![feature(ascii_char)]

use std::usize;

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

        checksum += l_file_id * (block_idx..block_idx+l_file_size as usize).sum::<usize>();
        block_idx += l_file_size as usize;

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

    let mut files = Vec::with_capacity(bytes.len()/2);
    let mut free = Vec::with_capacity(bytes.len()/2);
    let mut first_free_ge = [usize::MAX; 10];

    {
        let mut file_id = 0;
        let mut block_idx = 0;

        bytes[0..bytes.len()-2].iter()
            .map(|&c| (c - b'0') as usize)
            .tuples()
            .enumerate()
            .for_each(|(i, (file_size, free_size))| {
                files.push((file_id, file_size, block_idx));
                file_id += 1;
                block_idx += file_size;

                first_free_ge[free_size] = first_free_ge[free_size].min(i);
                free.push((free_size, block_idx));
                block_idx += free_size;
            });

        files.push((file_id, (bytes[bytes.len()-2] - b'0') as usize, block_idx));
    }

    for &(file_id, file_size, file_block_idx) in files.iter().rev() {
        let free_idx = first_free_ge[file_size];
        let start_block_idx = if free_idx < free.len() {
            let (free_size, free_block_idx) = free[free_idx];
            let remaining_free_size = free_size - file_size;
            free[free_idx].0 = remaining_free_size;
            free[free_idx].1 += file_size;

            for i in 1..free_size {
                if first_free_ge[i] >= free_idx {
                    first_free_ge[i] = free[free_idx..].iter().position(|&(size, _)| size >= i).unwrap_or(usize::MAX).saturating_add(free_idx);
                } else {
                    first_free_ge[i] = free[first_free_ge[i]..free_idx].iter().position(|&(size, _)| size >= i).unwrap_or(usize::MAX).saturating_add(first_free_ge[i]);
                }
            }

            first_free_ge[free_size] = free[free_idx..].iter().position(|&(size, _)| size >= free_size).unwrap_or(usize::MAX).saturating_add(free_idx);
            
            free_block_idx
        } else {
            file_block_idx
        };

        checksum += file_id * (start_block_idx..start_block_idx+file_size).sum::<usize>();
        free.pop();
    }

    checksum
}
