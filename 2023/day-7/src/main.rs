use core::fmt;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const LABEL_TO_STRENGTH_0: [usize; 36] = generate_label_lookup(b"23456789TJQKA");
const LABEL_TO_STRENGTH_1: [usize; 36] = generate_label_lookup(b"J23456789TQKA");

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
            (
                Hand::from_labels(hand, &LABEL_TO_STRENGTH_0),
                bet.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b))
        // .inspect(|(hand, _)| println!("{:?}", hand))
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum()
}

fn task_1(input: &str) -> usize {
    0
}

fn label_to_strength(label: u8, lookup: &[usize]) -> usize {
    lookup[(label - b'2') as usize]
}

const fn generate_label_lookup(label_order: &[u8; 13]) -> [usize; 36] {
    let mut map = [usize::MAX; 36];

    let mut i = 0;
    while i < 13 {
        map[(label_order[i] - b'2') as usize] = i;
        i += 1;
    }

    map
}

#[derive(PartialEq, Eq, Ord)]
struct Hand {
    kind: HandKind,
    strength: u64,
}

impl Hand {
    fn from_labels(s: &str, lookup: &[usize]) -> Self {
        let card_strengths = s
            .bytes()
            .map(|l| label_to_strength(l, lookup))
            .collect_vec();

        Self {
            kind: HandKind::from_label_strengths(&card_strengths),
            strength: Self::strength_from_label_strengths(&card_strengths),
        }
    }

    fn strength_from_label_strengths(s: &[usize]) -> u64 {
        s.iter()
            .rev()
            .enumerate()
            .map(|(i, s)| (*s as u64) << i * 8)
            .sum()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.kind
                .cmp(&other.kind)
                .then(self.strength.cmp(&other.strength)),
        )
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hand {{ kind: {:?}, strength: 0x{:010x} }}",
            self.kind, self.strength
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn from_label_strengths(strengths: &[usize]) -> Self {
        let mut counts = [0; 13];
        for s in strengths {
            counts[*s] += 1;
        }

        let mut pairs = 0;
        let mut tripple = false;
        for c in counts {
            match c {
                5 => return HandKind::FiveOfAKind,
                4 => return HandKind::FourOfAKind,
                3 => tripple = true,
                2 => pairs += 1,
                _ => (),
            }
        }

        match pairs {
            0 => {
                if tripple {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::HighCard
                }
            }
            1 => {
                if tripple {
                    HandKind::FullHouse
                } else {
                    HandKind::OnePair
                }
            }
            2 => HandKind::TwoPairs,
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
