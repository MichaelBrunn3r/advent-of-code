use std::collections::HashMap;

const ZZZ: u16 = encode_base_26("ZZZ");
const Z: u16 = 25;
const AAA: u16 = 0;

pub fn task_0(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let mut instructions = instructions.chars().cycle();

    let mut network = HashMap::<u16, (u16, u16)>::with_capacity(720);
    nodes.lines().for_each(|line| {
        let name = encode_base_26(&line[0..=2]);
        let left = encode_base_26(&line[7..=9]);
        let right = encode_base_26(&line[12..=14]);
        network.insert(name, (left, right));
    });

    let mut current = AAA;
    let mut step = 0;
    while current != ZZZ {
        let (left, right) = network.get(&current).unwrap();
        let instruction = instructions.next().unwrap();
        current = match instruction {
            'L' => *left,
            'R' => *right,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

pub fn task_1(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let mut instructions = instructions.chars().cycle();

    let mut network = HashMap::<u16, (u16, u16)>::with_capacity(720);
    let mut current_nodes = vec![];
    nodes.lines().for_each(|line| {
        let name = &line[0..=2];
        let key = encode_base_26(&line[0..=2]);

        if name.as_bytes()[2] == b'A' {
            current_nodes.push(key);
        }

        let left = encode_base_26(&line[7..=9]);
        let right = encode_base_26(&line[12..=14]);
        network.insert(key, (left, right));
    });

    let mut step = 0;
    while !current_nodes.iter().all(|&node| node & 0b11111 == Z) {
        let instruction = instructions.next().unwrap();

        for i in 0..current_nodes.len() {
            let (left, right) = network.get(&current_nodes[i]).unwrap();

            current_nodes[i] = match instruction {
                'L' => *left,
                'R' => *right,
                _ => unreachable!(),
            };
        }

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
