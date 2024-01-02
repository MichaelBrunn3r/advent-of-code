use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

static mut MODULES: [Module; 858] = unsafe { std::mem::zeroed() };

pub enum Module {
    FlipFlop(ArrayVec<u16, 5>),
    Conjunction(ArrayVec<u16, 5>),
}

impl Module {
    pub fn outputs(&self) -> &[u16] {
        match self {
            Module::FlipFlop(outputs) => outputs,
            Module::Conjunction(outputs) => outputs,
        }
    }
}

pub struct ModuleParser<'p> {
    data: &'p [u8],
    pub modules: &'p mut [Module; 858],
    pub broadcaster_outputs: ArrayVec<u16, 5>,
    pub cycle_conjunctions: ArrayVec<u16, 4>,
}

impl<'p> ModuleParser<'p> {
    pub fn new(data: &'p [u8]) -> Self {
        Self {
            data,
            modules: unsafe { &mut MODULES },
            broadcaster_outputs: ArrayVec::new(),
            cycle_conjunctions: ArrayVec::new(),
        }
    }

    fn count_conjunction_outputs(&mut self) -> usize {
        // #Outputs: {1:5, 4:4}
        if self.data[2] == b'\n' {
            1
        } else {
            5
        }
    }

    fn count_flipflip_outputs(&mut self) -> usize {
        // #Outputs: {1:16, 2:32}
        if self.data[2] == b'\n' {
            1
        } else {
            2
        }
    }

    fn parse_module_outputs(&mut self, num_outputs: usize) -> ArrayVec<u16, 5> {
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
        self.broadcaster_outputs = self.parse_module_outputs(4);
    }

    fn hash(&self, name: &[u8]) -> u16 {
        (name[0] - b'a') as u16 + (((name[1] - b'a') as u16) << 5)
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

            let name = &self.data[1..3];
            let hash = self.hash(name);
            self.data = &self.data["%aa -> ".len()..];

            match module_type {
                b'%' => {
                    let num_module_outputs = self.count_flipflip_outputs();
                    let module = Module::FlipFlop(self.parse_module_outputs(num_module_outputs));
                    self.modules[hash as usize] = module;
                }
                b'&' => {
                    let num_module_outputs = self.count_conjunction_outputs();
                    let module = Module::Conjunction(self.parse_module_outputs(num_module_outputs));
                    if num_module_outputs > 1 {
                        self.cycle_conjunctions.push(hash);
                        self.modules[hash as usize] = module;
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
