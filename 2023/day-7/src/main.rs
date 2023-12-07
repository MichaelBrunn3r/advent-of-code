use std::str::FromStr;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            (hand.parse::<Hand>().unwrap(), bet.parse::<usize>().unwrap())
        })
        // .inspect(|(hand, bet)| println!("{:?} {}", hand, bet))
        .sorted_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b))
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum()
}

fn task_1(input: &str) -> usize {
    0
}

fn label_to_strength(label: u8) -> usize {
    match label {
        b'2'..=b'9' => (label - b'2') as usize,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("Invalid label"),
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<usize>,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.bytes().map(label_to_strength).collect_vec();

        Ok(Self {
            hand_type: HandType::from_strengths(&cards),
            cards,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp_type = self.hand_type.cmp(&other.hand_type);
        if cmp_type != std::cmp::Ordering::Equal {
            return Some(cmp_type);
        }

        for (a, b) in self.cards.iter().zip(&other.cards) {
            if *a > *b {
                return Some(std::cmp::Ordering::Greater);
            } else if *a < *b {
                return Some(std::cmp::Ordering::Less);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// 0 pairs -> HighCard
// 1 pair  -> FullHouse, OnePair
// 2 pairs -> TwoPairs

impl HandType {
    fn from_strengths(strengths: &[usize]) -> Self {
        let mut counts = [0; 13];
        for s in strengths {
            counts[*s] += 1;
        }

        let mut pairs = 0;
        let mut tripple = false;
        for c in counts {
            match c {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => tripple = true,
                2 => pairs += 1,
                _ => (),
            }
        }

        match pairs {
            0 => {
                if tripple {
                    HandType::ThreeOfAKind
                } else {
                    HandType::HighCard
                }
            }
            1 => {
                if tripple {
                    HandType::FullHouse
                } else {
                    HandType::OnePair
                }
            }
            2 => HandType::TwoPairs,
            _ => panic!("Invalid number of pairs"),
        }
    }
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

    #[test]
    fn test_cmp_hands() {
        assert!("QQQ23".parse::<Hand>().unwrap() > "A3456".parse::<Hand>().unwrap());
        assert!("QQQ24".parse::<Hand>().unwrap() > "QQQ23".parse::<Hand>().unwrap());
    }
}
