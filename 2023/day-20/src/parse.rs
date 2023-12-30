use crate::{Module, Output, OFF};
use aoc::U8SliceExt;
use arrayvec::ArrayVec;
use core::num;
use std::collections::{hash_map::Entry, HashMap};

pub struct ModuleParser<'p> {
    data: &'p [u8],
    pub broadcaster_outputs: ArrayVec<Output, 5>,
    pub name_to_module_meta: HashMap<&'p [u8], ModuleMeta>,
    pub conjunctions: ArrayVec<u8, 9>,
}

impl<'p> ModuleParser<'p> {
    pub fn new(data: &'p [u8]) -> Self {
        Self {
            data,
            broadcaster_outputs: ArrayVec::new(),
            name_to_module_meta: HashMap::new(),
            conjunctions: ArrayVec::new(),
        }
    }

    fn num_outputs(&mut self) -> usize {
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

    fn parse_outputs(&mut self) -> ArrayVec<Output, 5> {
        let mut outputs = ArrayVec::new();
        for i in 0..self.num_outputs() - 1 {
            let meta = self.name_to_meta(&self.data[..2]);
            outputs.push(Output {
                id: meta.id,
                input_idx: meta.num_inputs,
            });
            meta.num_inputs += 1;
            self.data = &self.data["aa, ".len()..];
        }

        // Last output has no comma and ends with a newline
        let meta = self.name_to_meta(&self.data[..2]);
        outputs.push(Output {
            id: meta.id,
            input_idx: meta.num_inputs,
        });
        meta.num_inputs += 1;
        self.data = &self.data["aa\n".len()..];

        outputs
    }

    fn name_to_meta(&mut self, name: &'p [u8]) -> &mut ModuleMeta {
        let id = self.name_to_module_meta.len() as u8;
        self.name_to_module_meta
            .entry(name)
            .or_insert_with(|| ModuleMeta { id, num_inputs: 0 })
    }
}

impl<'p> Iterator for ModuleParser<'p> {
    type Item = (u8, Module);

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let mut module_type = self.data[0];

        if module_type == b'b' {
            self.data = &self.data["broadcaster -> ".len()..];
            self.broadcaster_outputs = self.parse_outputs();
            module_type = self.data[0];
        }

        let id = self.name_to_meta(&self.data[1..3]).id;
        self.data = &self.data["%aa -> ".len()..];

        let outputs = self.parse_outputs();
        match module_type {
            b'%' => Some((id, Module::FlipFlop(OFF, outputs))),
            b'&' => {
                self.conjunctions.push(id);
                Some((id, Module::Conjunction(0b1111_1111_1111_1111, outputs)))
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct ModuleMeta {
    pub id: u8,
    pub num_inputs: u8,
}
