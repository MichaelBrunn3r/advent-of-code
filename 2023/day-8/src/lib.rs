use aoc::prelude::UnsignedExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const LINE_LEN: usize = 17;
const NUM_POSSIBLE_NODES: usize = 26426;
const ZZZ: u16 = encode_base_26(b"ZZZ");
const Z: u16 = 25;
const AAA: u16 = 0;

type Network = [(u16, u16); NUM_POSSIBLE_NODES];
pub static mut NETWORK: Network = [(0, 0); NUM_POSSIBLE_NODES];
pub fn parse(input: &'_ str) -> (&'_ [u8], &'static Network, Vec<u16>) {
    let instructions = &input.as_bytes()[..283];

    let mut nodes_ending_in_a = vec![];
    input.as_bytes()[283 + "\n\n".len()..]
        .chunks_exact(LINE_LEN)
        .for_each(|line| {
            let name = &line[0..=2];
            let key = encode_base_26(name);
            if name[2] == b'A' {
                nodes_ending_in_a.push(key);
            }

            let left = encode_base_26(&line[7..=9]);
            let right = encode_base_26(&line[12..=14]);
            unsafe {
                NETWORK[key as usize] = (left, right);
            }
        });

    (instructions, unsafe { &NETWORK }, nodes_ending_in_a)
}

pub fn part_1(instructions: &[u8], network: &Network) -> usize {
    let mut instructions = instructions.iter().cycle();

    let mut current = AAA;
    let mut step = 0;
    while current != ZZZ {
        let (left, right) = network[current as usize];
        let instruction = instructions.next().unwrap();
        current = match instruction {
            b'L' => left,
            b'R' => right,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

pub fn part_2(instructions: &[u8], network: &Network, nodes_ending_in_a: &[u16]) -> usize {
    nodes_ending_in_a
        .par_iter()
        .map(|&node| calc_cycle_length(node, instructions, network))
        .reduce(|| 1, |a, b| a.lcm(b))
}

fn calc_cycle_length(start: u16, instructions: &[u8], network: &Network) -> usize {
    let mut instructions = instructions.iter().cycle();

    let mut current = start;
    let mut step = 0;
    while current & 0b11111 != Z {
        let (left, right) = network[current as usize];

        current = match instructions.next().unwrap() {
            b'L' => left,
            b'R' => right,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

const fn encode_base_26(name: &[u8]) -> u16 {
    let mut result = 0;
    result |= (name[2] - b'A') as u16;
    result |= ((name[1] - b'A') as u16) << 5;
    result |= ((name[0] - b'A') as u16) << 10;
    result
}
