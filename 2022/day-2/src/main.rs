use aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Total score: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

// Rock     = 1 beats 3
// Paper    = 2 beats 1
// Scissors = 3 beats 2
fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        // .inspect(|(left, right)| println!("{} {}", left, right))
        .map(|(left, right)| {
            (
                RPS::from(left.as_bytes()[0] - b'A'),
                RPS::from(right.as_bytes()[0] - b'X'),
            )
        })
        // .inspect(|(opponent, you)| println!("{:?} {:?}", opponent, you))
        .map(|(opponent, you)| {
            let mut score = you as usize + 1;
            if you == opponent {
                score += 3;
            } else if you.wins(&opponent) {
                score += 6;
            }
            score
        })
        // .inspect(|score| println!("score={}", score))
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        // .inspect(|(opponent, you)| println!("{} {}", opponent, you))
        .map(|(left, right)| {
            (
                RPS::from(left.as_bytes()[0] - b'A'),
                Strategy::from(right.as_bytes()[0] - b'X'),
            )
        })
        .map(|(opponent, strategy)| {
            let you: RPS = match strategy {
                Strategy::Draw => opponent,
                Strategy::Win => opponent.losses_to(),
                Strategy::Loose => opponent.wins_against(),
            };
            (opponent, you)
        })
        // .inspect(|(opponent, you)| println!("{:?} {:?}", opponent, you))
        .map(|(opponent, you)| {
            let mut score = you as usize + 1;
            if you == opponent {
                score += 3;
            } else if you.wins(&opponent) {
                score += 6;
            }
            score
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for RPS {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!("Invalid RPS value: {}", n),
        }
    }
}

impl RPS {
    fn wins(&self, other: &Self) -> bool {
        match self {
            Self::Rock => *other == Self::Scissors,
            Self::Paper => *other == Self::Rock,
            Self::Scissors => *other == Self::Paper,
        }
    }

    fn losses_to(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

enum Strategy {
    Loose,
    Draw,
    Win,
}

impl From<u8> for Strategy {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Loose,
            1 => Self::Draw,
            2 => Self::Win,
            _ => panic!("Invalid strategy value: {}", n),
        }
    }
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
