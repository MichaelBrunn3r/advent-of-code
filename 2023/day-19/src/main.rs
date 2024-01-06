#![allow(unused_imports, unused_variables)]

use aoc_2023_19::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    // println!("Part 1: {}", part_2(&aoc::read_example_to_string(0)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&aoc::read_input_to_string()), 418498)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&aoc::read_input_to_string()), 123331556462603)
    }
}
