use std::collections::HashMap;

const ZZZ: u16 = encode("ZZZ");
const AAA: u16 = encode("AAA");

pub fn task_0(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let mut instructions = instructions.chars().cycle();

    let mut network = HashMap::<u16, (u16, u16)>::with_capacity(720);
    nodes.lines().for_each(|line| {
        let name = encode(&line[0..=2]);
        let left = encode(&line[7..=9]);
        let right = encode(&line[12..=14]);
        network.insert(name, (left, right));
    });

    let mut current = AAA;
    let mut i = 0;
    while current != ZZZ {
        let (left, right) = network.get(&current).unwrap();
        let instruction = instructions.next().unwrap();
        current = match instruction {
            'L' => *left,
            'R' => *right,
            _ => unreachable!(),
        };

        i += 1;
    }

    i
}

pub fn task_1(input: &str) -> usize {
    0
}

const fn encode(name: &str) -> u16 {
    let name = name.as_bytes();
    let mut result = 0;
    result |= (name[0] - b'A') as u16;
    result |= ((name[1] - b'A') as u16) << 5;
    result |= ((name[2] - b'A') as u16) << 10;
    result
}
