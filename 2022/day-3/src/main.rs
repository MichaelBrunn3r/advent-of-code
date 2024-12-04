use aoc;
use core::panic;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> usize {
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

fn p2(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let (a, b, c) = chunk.next_tuple().unwrap();
            group_priority(a, b, c)
        })
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
        aoc::assert_solution(0, p1);
    }

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, p2);
    }
}
