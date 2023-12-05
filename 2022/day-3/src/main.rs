use core::panic;

use aoc;
use regex::Regex;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        // .inspect(|(left, right)| println!("{}--{}", left, right))
        .map(|(left, right)| {
            for c in left.chars() {
                if right.contains(c) {
                    return item_to_priority(c);
                }
            }
            panic!("Every rucksack has one item in both compartments");
        })
        .sum::<usize>()
}

fn item_to_priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - b'a' as usize + 1
    } else {
        c as usize - b'A' as usize + 27
    }
}

fn task_1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution(0, task_0);
    }

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, task_1);
    }
}
