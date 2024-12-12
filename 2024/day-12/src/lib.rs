use aoc::prelude::*;
use itertools::Itertools;

// b'A' = 0100_0001, b'Z' = 0101_1010
// Flags: ??VPPPPP, [P]lant, [V]isited

const MASK_PLANT: u8 = 0b0001_1111;
const FLAG_VISITED: u8 = 0b0010_0000;

pub fn p1(input: &mut str) -> usize {
    let bytes = unsafe { input.as_bytes_mut() };
    let line_length = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;
    let bytes_len = bytes.len();

    let mut total = 0;

    let mut unhandled_plots = vec![0usize];
    while let Some(idx_start_plot) = unhandled_plots.pop() {
        let plant = bytes[idx_start_plot];
        if plant & FLAG_VISITED != 0 || plant == b'\n' {
            continue;
        }
        bytes[idx_start_plot] |= FLAG_VISITED;

        let mut plot_area = 0;
        let mut stack = vec![idx_start_plot];
        let mut perimiter = 0;
        while let Some(idx_current) = stack.pop() {
            plot_area += 1;
            for idx_adjacent in [1i32, -1, line_length as i32, -(line_length as i32)]
                .iter()
                .map(|&offset| idx_current as i32 + offset)
            {
                if idx_adjacent < 0 || idx_adjacent >= bytes_len as i32 {
                    perimiter += 1;
                    continue;
                }

                let idx_adjacent = idx_adjacent as usize;
                let adjacent = bytes[idx_adjacent];

                if adjacent == b'\n' {
                    perimiter += 1;
                    continue;
                }

                if (plant & MASK_PLANT) == (adjacent & MASK_PLANT) {
                    if adjacent & FLAG_VISITED == 0 {
                        stack.push(idx_adjacent);
                    }
                    bytes[idx_adjacent] |= FLAG_VISITED;
                } else {
                    perimiter += 1;
                    if adjacent & FLAG_VISITED == 0 {
                        unhandled_plots.push(idx_adjacent);
                    }
                }
            }
        }

        total += plot_area * perimiter;
    }

    total
}

pub fn p2(input: &str) -> usize {
    0
}
