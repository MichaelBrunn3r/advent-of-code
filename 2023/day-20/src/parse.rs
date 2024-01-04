use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

pub static mut PARSER: ModuleParser = ModuleParser::new_const();

pub struct FlipFlop {
    pub outputs: [u16; 2],
}

pub struct ModuleParser {
    pub modules: [FlipFlop; 858],
    pub broadcaster_outputs: [u16; 4],
    pub cycle_conjunctions: [u16; 4],
    num_cycle_conjunctions: usize,
}

impl ModuleParser {
    pub const fn new_const() -> Self {
        Self {
            modules: unsafe { std::mem::zeroed() },
            broadcaster_outputs: [0; 4],
            cycle_conjunctions: [0; 4],
            num_cycle_conjunctions: 0,
        }
    }

    fn count_conjunction_outputs(data: &[u8]) -> usize {
        // #Outputs: {1:5, 4:4}
        if data[2] == b'\n' {
            1
        } else {
            5
        }
    }

    fn count_flipflip_outputs(data: &[u8]) -> usize {
        // #Outputs: {1:16, 2:32}
        if data[2] == b'\n' {
            1
        } else {
            2
        }
    }

    fn parse_module_outputs_inplace<'d, const N: usize>(
        num_outputs: usize,
        outputs: &mut [u16],
        mut data: &'d [u8],
    ) -> &'d [u8] {
        for i in 0..num_outputs - 1 {
            outputs[i] = Self::hash(&data[..2]);
            data = &data["aa, ".len()..];
        }

        // Last output has no comma and ends with a newline
        outputs[num_outputs - 1] = Self::hash(&data[..2]);
        data = &data["aa\n".len()..];

        data
    }

    fn hash(name: &[u8]) -> u16 {
        // b'a' + (b'a' << 5) = 0b1100_1000_0001
        name[0] as u16 + ((name[1] as u16) << 5) - 0b1100_1000_0001
    }

    fn reset(&mut self) {
        self.num_cycle_conjunctions = 0;
    }

    pub fn parse(&mut self, mut data: &[u8]) {
        self.reset();

        loop {
            if data.is_empty() {
                break;
            }

            match data[0] {
                b'b' => {
                    data = &data["broadcaster -> ".len()..];
                    data = Self::parse_module_outputs_inplace::<4>(
                        4,
                        &mut self.broadcaster_outputs,
                        data,
                    );
                }
                b'%' => {
                    let name = &data[1..3];
                    let hash = Self::hash(name);
                    data = &data["%aa -> ".len()..];

                    let num_module_outputs = Self::count_flipflip_outputs(data);

                    data = Self::parse_module_outputs_inplace::<2>(
                        num_module_outputs,
                        &mut self.modules[hash as usize].outputs,
                        data,
                    );
                }
                b'&' => {
                    let name = &data[1..3];
                    let hash = Self::hash(name);
                    data = &data["%aa -> ".len()..];

                    let num_module_outputs = Self::count_conjunction_outputs(data);
                    if num_module_outputs == 5 {
                        self.cycle_conjunctions[self.num_cycle_conjunctions] = hash; // Only store cycle conjunctions
                        self.num_cycle_conjunctions += 1;

                        data = &data["aa, bb, cc, dd, ee\n".len()..]; // Skip outputs
                    } else {
                        data = &data["aa\n".len()..]; // Skip other conjunctions
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
