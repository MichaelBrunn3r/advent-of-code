#![allow(unused_imports, unused_variables)]

mod parse;

use aoc::prelude::*;
use arrayvec::ArrayVec;
use itertools::Itertools;
use parse::ModuleParser;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Formatter,
    ops,
};

pub fn part_1(input: &str) -> usize {
    let (mut modules, broadcaster_outputs) = parse_input(input);

    let mut num_pulses = [0, 0];

    for i in 0..1000 {
        let mut queue =
            VecDeque::from_iter(broadcaster_outputs.iter().map(|output| (LOW, *output)));

        // Low pulse from the button to the broadcaster
        num_pulses[0] += 1;

        while !queue.is_empty() {
            let (pulse_in, output) = queue.pop_front().unwrap();
            num_pulses[pulse_in as usize] += 1;

            let module = &mut modules[output.id as usize];

            if let Some(pulse_out) = module.receive(pulse_in, output.input_idx) {
                let outputs = match module {
                    Module::FlipFlop(_, outputs) => outputs,
                    Module::Conjunction(_, outputs) => outputs,
                };

                for output in outputs.iter() {
                    queue.push_back((pulse_out, *output));
                }
            }
        }
    }

    num_pulses[0] * num_pulses[1]
}

pub fn part_2(input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> (ArrayVec<Module, 64>, ArrayVec<Output, 5>) {
    let mut parser = ModuleParser::new(input.as_bytes());
    let mut modules: ArrayVec<Module, 64> = ArrayVec::new();
    unsafe { modules.set_len(64) };
    let mut max_id = 0;
    loop {
        if let Some((id, module)) = parser.next() {
            max_id = max_id.max(id);
            modules[id as usize] = module;
        } else {
            break;
        }
    }
    unsafe { modules.set_len(max_id as usize + 1) };

    // The parser does not create an "rx" module, because it has no outputs.
    // Instead we have to add it manually
    if let Some(meta) = parser.name_to_module_meta.get("rx".as_bytes()) {
        unsafe { modules.set_len(modules.len().max(meta.id as usize + 1)) }
        modules[meta.id as usize] = Module::FlipFlop(OFF, ArrayVec::new());
    }

    for (name, meta) in parser.name_to_module_meta {
        match &mut modules[meta.id as usize] {
            Module::Conjunction(input_states, _) => {
                // Conjuntion modules can have up to 16 inputs. In order for them to work
                // properly, we need to set the unused inputs to HIGH.
                *input_states <<= meta.num_inputs;
            }
            _ => {}
        }
    }

    (modules, parser.broadcaster_outputs)
}

pub type Pulse = bool;
pub const HIGH: Pulse = true;
pub const LOW: Pulse = false;

pub type State = bool;
pub const ON: State = true;
pub const OFF: State = false;

pub enum Module {
    FlipFlop(State, ArrayVec<Output, 5>),
    Conjunction(u16, ArrayVec<Output, 5>),
}

#[derive(Debug, Clone, Copy)]
pub struct Output {
    id: u8,
    input_idx: u8,
}

impl Module {
    fn receive(&mut self, pulse: Pulse, input_idx: u8) -> Option<Pulse> {
        match self {
            Module::FlipFlop(state, outputs) => {
                if pulse == HIGH {
                    return None;
                }
                *state ^= !pulse;
                Some(*state)
            }
            Module::Conjunction(state, outputs) => {
                if pulse == HIGH {
                    *state |= 1 << input_idx;
                } else {
                    *state &= !(1 << input_idx);
                }
                Some(*state != 0b1111_1111_1111_1111)
            }
        }
    }
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Module::FlipFlop(state, outputs) => write!(
                f,
                "FlipFlop({}, {:?})",
                if *state { "ON" } else { "OFF" },
                outputs
            ),
            Module::Conjunction(state, outputs) => {
                let input_bits = format!("{:016b}", state);
                write!(
                    f,
                    "Conjunction(0b{}, {:?})",
                    input_bits
                        .chars()
                        .chunks(4)
                        .into_iter()
                        .map(|chunk| chunk.collect::<String>())
                        .join("_"),
                    outputs
                )
            }
        }
    }
}
