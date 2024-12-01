use aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Max total calories: {}", part_1(&input));
    println!("Total calories of top 3: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();
    elves.sort();
    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution(0, part_1);
    }

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, part_2);
    }
}
