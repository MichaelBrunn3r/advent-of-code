use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

pub static mut PARSER: ModuleParser = ModuleParser::new_const();

pub struct FlipFlop {
    pub outputs: [u16; 2],
}

pub struct ModuleParser {
    pub modules: [FlipFlop; 65536],
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

    unsafe fn count_conjunction_outputs(data: *const u8) -> usize {
        // #Outputs: {1:5, 4:4}
        if *data.offset(2) == b'\n' {
            1
        } else {
            5
        }
    }

    unsafe fn count_flipflop_outputs(data: *const u8) -> usize {
        // #Outputs: {1:16, 2:32}
        if *data.offset(2) == b'\n' {
            1
        } else {
            2
        }
    }

    unsafe fn parse_module_outputs_inplace<const N: usize>(
        num_outputs: usize,
        outputs: &mut [u16],
        mut data: *const u8,
    ) -> *mut u8 {
        outputs.iter_mut().take(num_outputs - 1).for_each(|output| {
            *output = Self::hash(data);
            data = data.add("aa, ".len());
        });

        // Last output has no comma and ends with a newline
        outputs[num_outputs - 1] = Self::hash(data);
        data = data.add("aa\n".len());

        data as *mut u8
    }

    unsafe fn hash(name: *const u8) -> u16 {
        (name as *const u16).read()
    }

    fn reset(&mut self) {
        self.num_cycle_conjunctions = 0;
    }

    pub fn parse(&mut self, data: &[u8]) {
        self.reset();
        let mut data = data.as_ptr();

        for _ in 0..58 {
            unsafe {
                match *data {
                    b'b' => {
                        data = data.add("broadcaster -> ".len());
                        data = Self::parse_module_outputs_inplace::<4>(
                            4,
                            &mut self.broadcaster_outputs,
                            data,
                        );
                    }
                    b'%' => {
                        let hash = Self::hash(data.offset(1));
                        data = data.add("%aa -> ".len());

                        let num_module_outputs = Self::count_flipflop_outputs(data);

                        data = Self::parse_module_outputs_inplace::<2>(
                            num_module_outputs,
                            &mut self.modules[hash as usize].outputs,
                            data,
                        );
                    }
                    b'&' => {
                        let hash = Self::hash(data.offset(1));
                        data = data.add("%aa -> ".len());

                        let num_module_outputs = Self::count_conjunction_outputs(data);
                        if num_module_outputs == 5 {
                            self.cycle_conjunctions[self.num_cycle_conjunctions] = hash; // Only store cycle conjunctions
                            self.num_cycle_conjunctions += 1;

                            data = data.add("aa, bb, cc, dd, ee\n".len());
                        // Skip outputs
                        } else {
                            data = data.add("aa\n".len()); // Skip other conjunctions
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
