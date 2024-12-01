pub mod parse;

use parse::{ModuleID, Modules};
use std::arch::asm;

// L = low pulse, H = high pulse, FF = FlipFlop
const N: usize = 1000;
const NUM_CYCLES: usize = 4;
const NUM_FFS_PER_CYCLE: u32 = 12;
const NUM_NOT_CONNECTED_FFS_PER_CYCLE: usize = 3;
const NUM_L_TO_BROADCASTER: usize = N;
const NUM_L_FROM_BROADCASTER: usize = NUM_CYCLES * N;
const NUM_L_BETWEEN_COUNTER_FFS: usize = N - N.count_ones() as usize;
const NUM_H_BETWEEN_COUNTER_FFS: usize = N;
const NUM_L_TO_CYCLE_CONJ: usize = calc_l_to_cycle_conjunction();
const NUM_H_TO_CYCLE_CONJ: usize = calc_h_to_cycle_conjunction(N);

pub fn part_1(
    broadcaster_outputs: &[ModuleID; 4],
    modules: &Modules,
    cycle_conjunctions: &[ModuleID; 4],
) -> usize {
    let mut num_low =
        NUM_L_TO_BROADCASTER + NUM_L_FROM_BROADCASTER + NUM_CYCLES * NUM_L_BETWEEN_COUNTER_FFS;
    let mut num_high = NUM_CYCLES * NUM_H_BETWEEN_COUNTER_FFS;

    for &broadcast_output in broadcaster_outputs {
        // Each broadcast output is the first FlipFlop in a cycle
        let start = &modules[broadcast_output as usize];

        // Find the conjunction of the cycle
        let (cycle_conj, mut next) = if cycle_conjunctions.contains(&start.outputs[0]) {
            (start.outputs[0], start.outputs[1])
        } else {
            (start.outputs[1], start.outputs[0])
        };

        let mut num_l_to_cycle_conj = NUM_L_TO_CYCLE_CONJ;
        let mut num_h_to_cycle_conj = NUM_H_TO_CYCLE_CONJ;

        let mut num_visited_not_connected_ffs = 0;
        for bit_idx in 2..=NUM_FFS_PER_CYCLE {
            let flipflop = &modules[next as usize];

            if flipflop.outputs[1] == 0 {
                num_l_to_cycle_conj -= N >> bit_idx;

                // Calculates: `num_h_to_cycle_conj -= round(N / 2^bit_idx)`
                // Divide by shifting right and then round by adding the carry bit.
                // Carry=1 indicates remainder >= 0.5, in which case we need to round up.
                let n = N;
                unsafe {
                    asm!(
                        "shr {n}, cl",
                        "sbb {num_h_to_cycle_conj}, {n}",
                        n = in(reg) n,
                        in("ecx") bit_idx,
                        num_h_to_cycle_conj = inout(reg) num_h_to_cycle_conj,
                    );
                }

                num_visited_not_connected_ffs += 1;
                if num_visited_not_connected_ffs == NUM_NOT_CONNECTED_FFS_PER_CYCLE {
                    break;
                }
            }

            next = if flipflop.outputs[0] == cycle_conj {
                flipflop.outputs[1]
            } else {
                flipflop.outputs[0]
            };
        }

        let num_pulses_to_cycle_conj = num_l_to_cycle_conj + num_h_to_cycle_conj;

        let num_l_from_cycle_conj = num_pulses_to_cycle_conj;
        let num_h_from_cycle_conj = 6 * num_pulses_to_cycle_conj;

        num_low += num_l_to_cycle_conj + num_l_from_cycle_conj;
        num_high += num_h_to_cycle_conj + num_h_from_cycle_conj;
    }

    num_low * num_high
}

const MAX_CYCLE_PERIOD: usize = 2usize.pow(NUM_FFS_PER_CYCLE);

pub fn part_2(
    broadcaster_outputs: &[ModuleID; 4],
    modules: &Modules,
    cycle_conjunctions: &[ModuleID; 4],
) -> usize {
    let mut result = 1;
    for &broadcast_output in broadcaster_outputs {
        // Each broadcast output is the first FlipFlop in a cycle
        let start = &modules[broadcast_output as usize];

        // Find the conjunction of the cycle
        let (cycle_conj, mut next) = if cycle_conjunctions.contains(&start.outputs[0]) {
            (start.outputs[0], start.outputs[1])
        } else {
            (start.outputs[1], start.outputs[0])
        };

        let mut cycle_period = MAX_CYCLE_PERIOD - 1;
        let mut num_visited_not_connected_ffs = 0usize; // FF = FlipFlop

        for bit_idx in 1..=NUM_FFS_PER_CYCLE {
            let flipflop = &modules[next as usize];

            if flipflop.outputs[1] == 0 {
                cycle_period -= 1 << bit_idx;

                num_visited_not_connected_ffs += 1;
                if num_visited_not_connected_ffs == NUM_NOT_CONNECTED_FFS_PER_CYCLE {
                    break;
                }
            }

            next = if flipflop.outputs[0] == cycle_conj {
                flipflop.outputs[1]
            } else {
                flipflop.outputs[0]
            };
        }

        result *= cycle_period
    }
    result
}

const fn calc_l_to_cycle_conjunction() -> usize {
    let mut sum = 0usize;
    let mut i = 1;
    while i <= 10 {
        sum += N / 2usize.pow(i);
        i += 1;
    }
    sum
}

const fn calc_h_to_cycle_conjunction(n: usize) -> usize {
    let mut sum = 0usize;
    let mut i = 1;
    while i <= 10 {
        sum += (n >> i) + (n & 1 << (i - 1) != 0) as usize;
        i += 1;
    }
    sum
}
