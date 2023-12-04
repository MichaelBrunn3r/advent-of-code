use aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Max total calories: {}", task_0(&input));
    println!("Total calories of top 3: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn task_1(input: &str) -> usize {
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
        if !aoc::solution_exists(0) {
            return;
        }

        let input = aoc::read_example_to_string(0);
        let expected = aoc::read_solution_to_string(0).parse::<usize>().unwrap();
        assert_eq!(task_0(&input), expected);
    }

    #[test]
    fn test_example_1() {
        if !aoc::solution_exists(1) {
            return;
        }

        let input = aoc::read_example_to_string(0);
        let expected = aoc::read_solution_to_string(1).parse::<usize>().unwrap();
        assert_eq!(task_1(&input), expected);
    }
}
