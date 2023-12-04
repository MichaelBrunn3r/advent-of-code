use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT_DIR: PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
}

fn main() {
    let input = std::fs::read_to_string(PROJECT_DIR.join("input")).unwrap();
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
        if !PROJECT_DIR.join("solution_0").exists() {
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

        let input = std::fs::read_to_string(PROJECT_DIR.join("example_1")).unwrap();
        let expected = std::fs::read_to_string(PROJECT_DIR.join("solution_1"))
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(task_1(&input), expected);
    }
}
