use aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Total score: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

// Rock     = 1 beats 3
// Paper    = 2 beats 1
// Scissors = 3 beats 2
fn task_0(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        // .inspect(|(opponent, you)| println!("{} {}", opponent, you))
        .map(|(opponent, you)| {
            (
                opponent.as_bytes()[0] - b'A' + 1,
                you.as_bytes()[0] - b'X' + 1,
            )
        })
        // .inspect(|(opponent, you)| println!("{} {}", opponent, you))
        .map(|(opponent, you)| {
            let mut score = you as usize;
            if you == opponent {
                score += 3;
            } else if ((you - 1 + 2) % 3) + 1 == opponent {
                score += 6;
            }
            score
        })
        // .inspect(|score| println!("score={}", score))
        .sum()
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
        assert_eq!(task_0(&input), expected);
    }
}
