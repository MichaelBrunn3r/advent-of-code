use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

static mut MODULES: [FlipFlop; 858] = unsafe { std::mem::zeroed() };

pub struct FlipFlop {
    pub outputs: ArrayVec<u16, 2>,
}

pub struct ModuleParser<'p> {
    data: &'p [u8],
    pub modules: &'p mut [FlipFlop; 858],
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

    fn parse_module_outputs_inplace<const N: usize>(
        num_outputs: usize,
        outputs: &mut ArrayVec<u16, N>,
        mut data: &'p [u8],
    ) -> &'p [u8] {
        outputs.clear();
        for i in 0..num_outputs - 1 {
            unsafe { outputs.push_unchecked(Self::hash(&data[..2])) };
            data = &data["aa, ".len()..];
        }

        // Last output has no comma and ends with a newline
        unsafe { outputs.push_unchecked(Self::hash(&data[..2])) };
        data = &data["aa\n".len()..];

        data
    }

    fn parse_broadcaster(&mut self) {
        self.data = &self.data["broadcaster -> ".len()..];
        self.data =
            Self::parse_module_outputs_inplace::<5>(4, &mut self.broadcaster_outputs, self.data);
    }

    fn hash(name: &[u8]) -> u16 {
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
            let hash = Self::hash(name);
            self.data = &self.data["%aa -> ".len()..];

            match module_type {
                b'%' => {
                    let num_module_outputs = self.count_flipflip_outputs();
                    self.data = Self::parse_module_outputs_inplace::<2>(
                        num_module_outputs,
                        &mut self.modules[hash as usize].outputs,
                        self.data,
                    );
                }
                b'&' => {
                    let num_module_outputs = self.count_conjunction_outputs();
                    if num_module_outputs == 5 {
                        unsafe { self.cycle_conjunctions.push_unchecked(hash) }; // Only store cycle conjunctions
                        self.data = &self.data["aa, bb, cc, dd, ee\n".len()..]; // Skip outputs
                    } else {
                        self.data = &self.data["aa\n".len()..]; // Skip other conjunctions
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
