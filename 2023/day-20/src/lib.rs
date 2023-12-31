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

const NUM_FFS_PER_CYCLE: usize = 12; // FF = FlipFlop
const MAX_CYCLE_LEN: usize = 4096;
const NUM_NOT_CONNECTED_FFS_PER_CYCLE: usize = 4;

pub fn part_1(input: &str) -> usize {
    // let (mut modules, rx_id, broadcaster_outputs, _) = parse_input(input);

    // let mut num_pulses = [0, 0];

    // let mut queue = VecDeque::new();
    // for i in 0..1000 {
    //     queue.extend(broadcaster_outputs.iter().map(|output| (LOW, *output)));

    //     // Low pulse from the button to the broadcaster
    //     num_pulses[0] += 1;

    //     while !queue.is_empty() {
    //         let (pulse_in, output) = queue.pop_front().unwrap();
    //         num_pulses[pulse_in as usize] += 1;

    //         let module = &mut modules[output.id as usize];

    //         if let Some(pulse_out) = module.receive(pulse_in, output.input_idx) {
    //             let outputs = match module {
    //                 Module::FlipFlop(_, outputs) => outputs,
    //                 Module::Conjunction(_, outputs) => outputs,
    //             };

    //             for output in outputs.iter() {
    //                 queue.push_back((pulse_out, *output));
    //             }
    //         }
    //     }
    // }

    // num_pulses[0] * num_pulses[1]
    0
}

pub fn part_2(input: &str) -> usize {
    let mut parser = ModuleParser::new(input.as_bytes());
    parser.parse();

    parser
        .broadcaster_outputs
        .iter()
        .map(|broadcast_output| {
            // Each broadcast output is the first FlipFlop in a cycle
            let start = parser.modules.get(broadcast_output).unwrap();
            let start_outputs = start.outputs();

            // Find the conjunction of the cycle
            let (cycle_conj, mut next) = if parser.cycle_conjunctions.contains(&start_outputs[0]) {
                (start_outputs[0], start_outputs[1])
            } else {
                (start_outputs[1], start_outputs[0])
            };

            let mut sum_not_connected = 1usize;
            let mut num_not_connected_ffs = 1usize; // FF = FlipFlop

            for bit_idx in 1..=NUM_FFS_PER_CYCLE {
                let module = parser.modules.get(next).unwrap();
                let outputs = module.outputs();

                if outputs.len() == 1 {
                    sum_not_connected += 1 << bit_idx;
                    num_not_connected_ffs += 1;

                    if num_not_connected_ffs == NUM_NOT_CONNECTED_FFS_PER_CYCLE {
                        break;
                    }
                }

                next = if outputs[0] == cycle_conj {
                    outputs[1]
                } else {
                    outputs[0]
                };
            }

            MAX_CYCLE_LEN - sum_not_connected
        })
        .reduce(|a, b| a.lcm(b))
        .unwrap()
}

pub enum Module<'m> {
    FlipFlop(ArrayVec<&'m str, 5>),
    Conjunction(ArrayVec<&'m str, 5>),
}

impl<'m> Module<'m> {
    fn outputs(&self) -> &[&'m str] {
        match self {
            Module::FlipFlop(outputs) => outputs,
            Module::Conjunction(outputs) => outputs,
        }
    }
}
