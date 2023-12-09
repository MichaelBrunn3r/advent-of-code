use aoc::prelude::UnsignedExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const NUM_POSSIBLE_NODES: usize = 26426;
const ZZZ: u16 = encode_base_26("ZZZ");
const Z: u16 = 25;
const AAA: u16 = 0;

pub static mut NETWORK: [(u16, u16); NUM_POSSIBLE_NODES] = [(0, 0); NUM_POSSIBLE_NODES];

pub fn part_1(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let mut instructions = instructions.chars().cycle();

    nodes.lines().for_each(|line| {
        let name = encode_base_26(&line[0..=2]);
        let left = encode_base_26(&line[7..=9]);
        let right = encode_base_26(&line[12..=14]);
        unsafe {
            NETWORK[name as usize] = (left, right);
        }
    });

    let mut current = AAA;
    let mut step = 0;
    while current != ZZZ {
        let (left, right) = unsafe { NETWORK[current as usize] };
        let instruction = instructions.next().unwrap();
        current = match instruction {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

pub fn part_2(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().cycle();

    let mut current_nodes = vec![];
    nodes.lines().for_each(|line| {
        let name = &line[0..=2];
        let key = encode_base_26(&line[0..=2]);

        if name.as_bytes()[2] == b'A' {
            current_nodes.push(key);
        }

        let left = encode_base_26(&line[7..=9]);
        let right = encode_base_26(&line[12..=14]);
        unsafe {
            NETWORK[key as usize] = (left, right);
        }
    });

    current_nodes
        .par_iter()
        .map(|&node| calc_cycle_length(node, instructions.clone()))
        .reduce(|| 1, |a, b| a.lcm(b))
}

fn calc_cycle_length(start: u16, mut instructions: impl Iterator<Item = char>) -> usize {
    let mut current = start;
    let mut step = 0;
    while current & 0b11111 != Z {
        let (left, right) = unsafe { NETWORK[current as usize] };

        current = match instructions.next().unwrap() {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

const fn encode_base_26(name: &str) -> u16 {
    let name = name.as_bytes();
    let mut result = 0;
    result |= (name[2] - b'A') as u16;
    result |= ((name[1] - b'A') as u16) << 5;
    result |= ((name[0] - b'A') as u16) << 10;
    result
}

fn decode_base_26(name: u16) -> String {
    let mut result = [0u8; 3];
    result[2] = (name & 0b11111) as u8 + b'A';
    result[1] = ((name >> 5) & 0b11111) as u8 + b'A';
    result[0] = ((name >> 10) & 0b11111) as u8 + b'A';

    String::from_utf8(result.to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ends_in_z() {
        assert!(encode_base_26("KKZ") & 0b11111 == Z);
    }
}
