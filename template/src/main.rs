use aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    0
}

fn task_1(input: &str) -> usize {
    0
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

        let input = aoc::read_example_to_string(1);
        let expected = aoc::read_solution_to_string(1).parse::<usize>().unwrap();
        assert_eq!(task_1(&input), expected);
    }
}
