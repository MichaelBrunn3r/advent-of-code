#![allow(unused_imports, unused_variables)]

use aoc_2023_4::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&aoc::read_input_to_string()), 23678);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&aoc::read_input_to_string()), 15455663);
    }
}
