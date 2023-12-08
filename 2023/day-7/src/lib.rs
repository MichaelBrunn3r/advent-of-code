use core::fmt;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const LABEL_TO_STRENGTH_0: [u32; 36] = generate_label_to_strength_lookup(b"23456789TJQKA");
const LABEL_TO_STRENGTH_1: [u32; 36] = generate_label_to_strength_lookup(b"J23456789TQKA");

pub fn task_0(input: &str) -> usize {
    count_winnings(input, &LABEL_TO_STRENGTH_0, false) as usize
}

pub fn task_1(input: &str) -> usize {
    count_winnings(input, &LABEL_TO_STRENGTH_1, true) as usize
}

fn count_winnings(input: &str, lookup: &[u32; 36], joker_is_wildcard: bool) -> u32 {
    input
        .lines()
        .map(|line| {
            let hand_strength = labels_to_hand_strength(&line[..5], lookup, joker_is_wildcard);
            let bet = str_to_u32(&line[6..]);
            (hand_strength, bet)
        })
        .sorted_by_cached_key(|(hand_strength, _)| *hand_strength)
        .zip(1..)
        .map(|((_, bet), i)| i * bet)
        .sum()
}

pub fn str_to_u32(bid: &str) -> u32 {
    let mut val = 0u32;

    for c in bid.bytes() {
        val = val * 10 + (c - b'0') as u32;
    }

    val
}

const fn generate_label_to_strength_lookup(label_order: &[u8; 13]) -> [u32; 36] {
    let mut map = [u32::MAX; 36];

    let mut i = 0;
    while i < 13 {
        map[(label_order[i] - b'2') as usize] = i as u32;
        i += 1;
    }

    map
}

fn labels_to_hand_strength(labels: &str, lookup: &[u32; 36], joker_is_wildcard: bool) -> u32 {
    let kind = HandKind::from_labels(labels, lookup, joker_is_wildcard);

    labels
        .bytes()
        .rev()
        .map(|l| lookup[l as usize - b'2' as usize])
        .enumerate()
        .map(|(i, s)| (s as u32) << (i << 2)) // i * 4
        .sum::<u32>()
        | kind as u32
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u32)]
pub enum HandKind {
    HighCard = 0 << 5 * 4,
    OnePair = 1 << 5 * 4,
    TwoPairs = 2 << 5 * 4,
    ThreeOfAKind = 3 << 5 * 4,
    FullHouse = 4 << 5 * 4,
    FourOfAKind = 5 << 5 * 4,
    FiveOfAKind = 6 << 5 * 4,
}

impl HandKind {
    fn from_labels(labels: &str, lookup: &[u32; 36], joker_is_wildcard: bool) -> Self {
        let mut counts = [0; 13];
        let mut max = 0;
        let mut max_idx = 0usize;

        for l in labels.chars() {
            let strength = lookup[l as usize - b'2' as usize] as usize;
            let count = counts[strength] + 1;

            if count >= max && !(joker_is_wildcard && l == 'J') {
                max = count;
                max_idx = strength;
            }

            counts[strength] = count;
        }

        if joker_is_wildcard {
            let joker_strength = lookup[b'J' as usize - (b'2' as usize)] as usize;
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
        assert!(
            labels_to_hand_strength("QQQ23", &LABEL_TO_STRENGTH_0, false)
                > labels_to_hand_strength("A3456", &LABEL_TO_STRENGTH_0, false)
        );
        assert!(
            labels_to_hand_strength("QQQ24", &LABEL_TO_STRENGTH_0, false)
                > labels_to_hand_strength("QQQ23", &LABEL_TO_STRENGTH_0, false)
        );
    }

    #[test]
    fn test_kind_from_labels() {
        assert_eq!(
            HandKind::from_labels("QQQ23", &LABEL_TO_STRENGTH_0, false),
            HandKind::ThreeOfAKind
        );

        assert_eq!(
            HandKind::from_labels("2345J", &LABEL_TO_STRENGTH_1, true),
            HandKind::OnePair
        );
    }
}
