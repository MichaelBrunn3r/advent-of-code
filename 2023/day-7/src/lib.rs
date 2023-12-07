use core::fmt;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const LABEL_TO_STRENGTH_0: [usize; 36] = generate_label_lookup(b"23456789TJQKA");
const LABEL_TO_STRENGTH_1: [usize; 36] = generate_label_lookup(b"J23456789TQKA");

pub fn task_0(input: &str) -> usize {
    count_winnings(input, &Rules::new(&LABEL_TO_STRENGTH_0, false))
}

pub fn task_1(input: &str) -> usize {
    count_winnings(input, &Rules::new(&LABEL_TO_STRENGTH_1, true))
}

fn count_winnings(input: &str, rules: &Rules) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();
            (
                Hand::from_labels(hand, &rules),
                bet.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b))
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum()
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

struct Rules<'a> {
    label_to_strength: &'a [usize; 36],
    joker_is_wildcard: bool,
}

impl<'a> Rules<'a> {
    fn new(label_to_strength: &'a [usize; 36], joker_is_wildcard: bool) -> Self {
        Self {
            label_to_strength,
            joker_is_wildcard,
        }
    }

    fn label_to_strength(&self, label: u8) -> usize {
        self.label_to_strength[(label - b'2') as usize]
    }
}

#[derive(PartialEq, Eq, Ord)]
struct Hand {
    kind: HandKind,
    strength: u64,
}

impl Hand {
    fn from_labels(labels: &str, rules: &Rules) -> Self {
        let card_strengths = labels
            .bytes()
            .map(|l| rules.label_to_strength(l))
            .collect_vec();

        Self {
            kind: HandKind::from_labels(labels, rules),
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
    fn from_labels(labels: &str, rules: &Rules) -> Self {
        let mut counts = [0; 13];
        let mut max = 0;
        let mut max_idx = 0;

        for l in labels.chars() {
            let strength = rules.label_to_strength(l as u8);
            let count = counts[strength] + 1;

            if count >= max && !(rules.joker_is_wildcard && l == 'J') {
                max = count;
                max_idx = strength;
            }

            counts[strength] = count;
        }

        if rules.joker_is_wildcard {
            let joker_strength = rules.label_to_strength(b'J');
            let joker_count = counts[joker_strength];
            counts[joker_strength] = 0;
            counts[max_idx] += joker_count;
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
    fn test_cmp_hands() {
        let no_joker = Rules {
            label_to_strength: &LABEL_TO_STRENGTH_0,
            joker_is_wildcard: false,
        };

        assert!(Hand::from_labels("QQQ23", &no_joker) > Hand::from_labels("A3456", &no_joker));
        assert!(Hand::from_labels("QQQ24", &no_joker) > Hand::from_labels("QQQ23", &no_joker));
    }

    #[test]
    fn test_kind_from_labels() {
        let no_joker = Rules {
            label_to_strength: &LABEL_TO_STRENGTH_0,
            joker_is_wildcard: false,
        };

        assert_eq!(
            HandKind::from_labels("QQQ23", &no_joker),
            HandKind::ThreeOfAKind
        );

        let joker = Rules {
            label_to_strength: &LABEL_TO_STRENGTH_1,
            joker_is_wildcard: true,
        };

        assert_eq!(HandKind::from_labels("2345J", &joker), HandKind::OnePair);
    }
}
