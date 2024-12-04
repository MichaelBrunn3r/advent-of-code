#![allow(unused_imports, unused_variables)]

use aoc_2023_4::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_input_to_string()), 23678);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_input_to_string()), 15455663);
    }
}
