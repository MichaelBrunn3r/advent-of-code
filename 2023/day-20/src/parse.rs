use crate::Module;
use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

pub struct ModuleParser<'p> {
    data: &'p [u8],
    pub modules: [Module; 676],
    pub broadcaster_outputs: ArrayVec<usize, 5>,
    pub cycle_conjunctions: ArrayVec<usize, 4>,
}

impl<'p> ModuleParser<'p> {
    pub fn new(data: &'p [u8]) -> Self {
        Self {
            data,
            modules: unsafe { std::mem::zeroed() },
            broadcaster_outputs: ArrayVec::new(),
            cycle_conjunctions: ArrayVec::new(),
        }
    }

    fn count_module_outputs(&mut self) -> usize {
        if self.data[2] == b'\n' {
            1
        } else if self.data[6] == b'\n' {
            2
        } else if self.data[18] == b'\n' {
            5
        } else if self.data[14] == b'\n' {
            4
        } else {
            3
        }
    }

    fn parse_module_outputs(&mut self, num_outputs: usize) -> ArrayVec<usize, 5> {
        let mut outputs = ArrayVec::new();
        for i in 0..num_outputs - 1 {
            outputs.push(self.hash(&self.data[..2]));
            self.data = &self.data["aa, ".len()..];
        }

        // Last output has no comma and ends with a newline
        outputs.push(self.hash(&self.data[..2]));
        self.data = &self.data["aa\n".len()..];

        outputs
    }

    fn parse_broadcaster(&mut self) {
        self.data = &self.data["broadcaster -> ".len()..];
        let num_module_outputs = self.count_module_outputs();
        self.broadcaster_outputs = self.parse_module_outputs(num_module_outputs);
    }

    fn hash(&self, name: &[u8]) -> usize {
        (name[0] - b'a') as usize + ((name[1] - b'a') as usize) * 26
    }

    pub fn parse(&mut self) {
        loop {
            if self.data.is_empty() {
                break;
            }

            if self.data[0] == b'b' {
                self.parse_broadcaster();
            };

            let module_type = self.data[0];

            let hash = self.hash(&self.data[1..3]);
            self.data = &self.data["%aa -> ".len()..];

            let num_module_outputs = self.count_module_outputs();
            match module_type {
                b'%' => {
                    let module = Module::FlipFlop(self.parse_module_outputs(num_module_outputs));
                    self.modules[hash] = module;
                }
                b'&' => {
                    let module = Module::Conjunction(self.parse_module_outputs(num_module_outputs));
                    if num_module_outputs > 1 {
                        self.cycle_conjunctions.push(hash);
                        self.modules[hash] = module;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug)]
pub struct ModuleMeta {
    pub id: u8,
    pub num_inputs: u8,
}
