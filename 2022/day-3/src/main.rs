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
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks_exact(3)
        .map(|chunk| {
            let (a, b, c) = (chunk[0], chunk[1], chunk[2]);
            group_priority(a, b, c)
        })
        // .inspect(|priority| println!("{}", priority))
        .sum::<usize>()
}

fn group_priority(a: &str, b: &str, c: &str) -> usize {
    let mut a_priorities = 0usize;
    for c in a.chars() {
        a_priorities |= 1 << item_to_priority(c);
    }

    let mut b_priorities = 0usize;
    for c in b.chars() {
        b_priorities |= 1 << item_to_priority(c);
    }

    let mut c_priorities = 0usize;
    for c in c.chars() {
        c_priorities |= 1 << item_to_priority(c);
    }

    let mut common_priorities = a_priorities & b_priorities & c_priorities;

    let mut priority = 0usize;
    while common_priorities > 1 {
        common_priorities >>= 1;
        priority += 1;
    }

    priority
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
