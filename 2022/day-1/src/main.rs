use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT_DIR: PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
}

fn main() {
    let input = std::fs::read_to_string(PROJECT_DIR.join("input")).unwrap();
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
        if !PROJECT_DIR.join("example_0").exists() {
            return;
        }

        let input = std::fs::read_to_string(PROJECT_DIR.join("example_0")).unwrap();
        let expected = std::fs::read_to_string(PROJECT_DIR.join("solution_0"))
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(task_0(&input), expected);
    }

    #[test]
    fn test_example_1() {
        if !PROJECT_DIR.join("solution_1").exists() {
            return;
        }

        let input = std::fs::read_to_string(PROJECT_DIR.join("example_0")).unwrap();
        let expected = std::fs::read_to_string(PROJECT_DIR.join("solution_1"))
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(task_1(&input), expected);
    }
}
