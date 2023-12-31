use crate::Module;
use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

pub struct ModuleParser<'p> {
    data: &'p [u8],
    pub modules: HashMap<&'p str, Module<'p>>,
    pub broadcaster_outputs: ArrayVec<&'p str, 5>,
    pub cycle_conjunctions: ArrayVec<&'p str, 4>,
}

impl<'p> ModuleParser<'p> {
    pub fn new(data: &'p [u8]) -> Self {
        Self {
            data,
            modules: HashMap::new(),
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

    fn parse_module_outputs(&mut self, num_outputs: usize) -> ArrayVec<&'p str, 5> {
        let mut outputs = ArrayVec::new();
        for i in 0..num_outputs - 1 {
            outputs.push(self.data[..2].as_str_unchecked());
            self.data = &self.data["aa, ".len()..];
        }

        // Last output has no comma and ends with a newline
        outputs.push(self.data[..2].as_str_unchecked());
        self.data = &self.data["aa\n".len()..];

        outputs
    }

    fn parse_broadcaster(&mut self) {
        self.data = &self.data["broadcaster -> ".len()..];
        let num_module_outputs = self.count_module_outputs();
        self.broadcaster_outputs = self.parse_module_outputs(num_module_outputs);
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

            let name = &self.data[1..3].as_str_unchecked();
            self.data = &self.data["%aa -> ".len()..];

            let num_module_outputs = self.count_module_outputs();
            match module_type {
                b'%' => {
                    let module = Module::FlipFlop(self.parse_module_outputs(num_module_outputs));
                    self.modules.insert(name, module);
                }
                b'&' => {
                    let module = Module::Conjunction(self.parse_module_outputs(num_module_outputs));
                    if num_module_outputs > 1 {
                        self.cycle_conjunctions.push(name);
                        self.modules.insert(name, module);
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
