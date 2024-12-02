#![allow(unused_imports, unused_variables)]

use aoc::ConstVec;
use aoc_2024_1::*;

fn main() {
    let input = aoc::read_input_to_string();
    let mut left: [u32; 1000] = unsafe { std::mem::zeroed() };
    let mut right: [u32; 1000] = unsafe { std::mem::zeroed() };
    parse(&input, &mut left, &mut right);

    println!("Part 1: {}", part_1(&mut left, &mut right));
    println!("Part 2: {}", part_2(&mut left, &mut right));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut left: [u32; 1000] = unsafe { std::mem::zeroed() };
        let mut right: [u32; 1000] = unsafe { std::mem::zeroed() };
        parse(&aoc::read_input_to_string(), &mut left, &mut right);

        assert_eq!(part_1(&mut left, &mut right), 1320851);
    }

    #[test]
    fn test_part_2() {
        let mut left: [u32; 1000] = unsafe { std::mem::zeroed() };
        let mut right: [u32; 1000] = unsafe { std::mem::zeroed() };
        parse(&aoc::read_input_to_string(), &mut left, &mut right);

        left.sort_unstable();
        right.sort_unstable();

        assert_eq!(part_2(&mut left, &mut right), 26859182);
    }
}
