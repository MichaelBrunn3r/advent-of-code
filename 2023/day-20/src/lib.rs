#![allow(unused_imports, unused_variables)]

mod parse;

use aoc::prelude::*;
use arrayvec::ArrayVec;
use core::num;
use itertools::Itertools;
use parse::ModuleParser;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Formatter,
    ops,
};

// L = low pulse, H = high pulse, FF = FlipFlop
const N: usize = 1000;
const NUM_CYCLES: usize = 4;
const NUM_FFS_PER_CYCLE: u32 = 12;
const NUM_NOT_CONNECTED_FFS_PER_CYCLE: usize = 3;
const NUM_L_TO_BROADCASTER: usize = N;
const NUM_L_FROM_BROADCASTER: usize = NUM_CYCLES * N;
const NUM_L_BETWEEN_COUNTER_FFS: usize = N - N.count_ones() as usize;
const NUM_H_BETWEEN_COUNTER_FFS: usize = N;
const NUM_L_TO_CYCLE_CONJ: usize = calc_l_to_cycle_conjunction(N);
const NUM_H_TO_CYCLE_CONJ: usize = calc_h_to_cycle_conjunction(N);

pub fn part_1(input: &str) -> usize {
    let mut num_low =
        NUM_L_TO_BROADCASTER + NUM_L_FROM_BROADCASTER + NUM_CYCLES * NUM_L_BETWEEN_COUNTER_FFS;
    let mut num_high = NUM_CYCLES * NUM_H_BETWEEN_COUNTER_FFS;

    let mut parser = ModuleParser::new(input.as_bytes());
    parser.parse();

    let (num_l_to_from_cycle_conjunctions, num_h_to_from_cycle_conjunctions) = parser
        .broadcaster_outputs
        .iter()
        .map(|&broadcast_output| {
            // Each broadcast output is the first FlipFlop in a cycle
            let start = &parser.modules[broadcast_output as usize];
            let start_outputs = start.outputs();

            // Find the conjunction of the cycle
            let (cycle_conj, mut next) = if parser.cycle_conjunctions.contains(&start_outputs[0]) {
                (start_outputs[0], start_outputs[1])
            } else {
                (start_outputs[1], start_outputs[0])
            };

            let mut num_l_to_cycle_conj = NUM_L_TO_CYCLE_CONJ;
            let mut num_h_to_cycle_conj = NUM_H_TO_CYCLE_CONJ;

            let mut num_visited_not_connected_ffs = 0;
            for bit_idx in 2..=NUM_FFS_PER_CYCLE {
                let module = &parser.modules[next as usize];
                let outputs = module.outputs();

                if outputs.len() == 1 {
                    num_l_to_cycle_conj -= N / 2usize.pow(bit_idx);
                    num_h_to_cycle_conj -= round_integer_division(N, 2usize.pow(bit_idx));

                    num_visited_not_connected_ffs += 1;
                    if num_visited_not_connected_ffs == NUM_NOT_CONNECTED_FFS_PER_CYCLE {
                        break;
                    }
                }

                next = if outputs[0] == cycle_conj {
                    outputs[1]
                } else {
                    outputs[0]
                };
            }

            let num_pulses_to_cycle_conj = num_l_to_cycle_conj + num_h_to_cycle_conj;

            let num_l_from_cycle_conj = num_pulses_to_cycle_conj;
            let num_h_from_cycle_conj = 6 * num_pulses_to_cycle_conj;

            (
                num_l_to_cycle_conj + num_l_from_cycle_conj,
                num_h_to_cycle_conj + num_h_from_cycle_conj,
            )
        })
        .reduce(|(acc_l, acc_h), (l, h)| (acc_l + l, acc_h + h))
        .unwrap();

    num_low += num_l_to_from_cycle_conjunctions;
    num_high += num_h_to_from_cycle_conjunctions;

    num_low * num_high
}

const MAX_CYCLE_PERIOD: usize = 2usize.pow(NUM_FFS_PER_CYCLE);

pub fn part_2(input: &str) -> usize {
    let mut parser = ModuleParser::new(input.as_bytes());
    parser.parse();

    parser
        .broadcaster_outputs
        .iter()
        .map(|&broadcast_output| {
            // Each broadcast output is the first FlipFlop in a cycle
            let start = &parser.modules[broadcast_output as usize];
            let start_outputs = start.outputs();

            // Find the conjunction of the cycle
            let (cycle_conj, mut next) = if parser.cycle_conjunctions.contains(&start_outputs[0]) {
                (start_outputs[0], start_outputs[1])
            } else {
                (start_outputs[1], start_outputs[0])
            };

            let mut cycle_period = MAX_CYCLE_PERIOD - 1;
            let mut num_visited_not_connected_ffs = 0usize; // FF = FlipFlop

            for bit_idx in 1..=NUM_FFS_PER_CYCLE {
                let module = &parser.modules[next as usize];
                let outputs = module.outputs();

                if outputs.len() == 1 {
                    cycle_period -= 1 << bit_idx;

                    num_visited_not_connected_ffs += 1;
                    if num_visited_not_connected_ffs == NUM_NOT_CONNECTED_FFS_PER_CYCLE {
                        break;
                    }
                }

                next = if outputs[0] == cycle_conj {
                    outputs[1]
                } else {
                    outputs[0]
                };
            }

            cycle_period
        })
        .product() // Product = LCM, because cycle periods are co-prime
}

const fn calc_l_to_cycle_conjunction(n: usize) -> usize {
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
        sum += round_integer_division(N, 2usize.pow(i));
        i += 1;
    }
    sum
}

const fn round_integer_division(numerator: usize, denominator: usize) -> usize {
    let div = (numerator * 10) / denominator;

    if div % 10 >= 5 {
        (div / 10) + 1
    } else {
        div / 10
    }
}
