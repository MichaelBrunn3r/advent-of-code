pub mod lut;

use aoc::StrExt;
use itertools::Itertools;
use lut::*;

pub const CARD_LUT_LEN: usize = 36;
pub const SMALLEST_LABEL: usize = b'2' as usize;
const NUM_LABELS: usize = 13;

pub fn part_1(input: &str) -> usize {
    count_winnings(
        input,
        &LABEL_TO_STRENGTH_LUT_0,
        &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0,
    ) as usize
}

pub fn part_2(input: &str) -> usize {
    count_winnings(
        input,
        &LABEL_TO_STRENGTH_LUT_1,
        &COUNT_OCCURENCES_TO_HAND_KIND_LUT_1,
    ) as usize
}

fn count_winnings(
    input: &str,
    card_lut: &[u32; CARD_LUT_LEN],
    occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
) -> u32 {
    input
        .lines()
        .map(|line| {
            let hand_strength = labels_to_hand_strength(&line[..5], card_lut, occurences_lut);
            let bet = line[6..].parse_u32_unchecked();
            (hand_strength, bet)
        })
        .sorted_by_cached_key(|(hand_strength, _)| *hand_strength)
        .zip(1..)
        .map(|((_, bet), i)| i * bet)
        .sum()
}

fn labels_to_hand_strength(
    labels: &str,
    card_lut: &[u32; CARD_LUT_LEN],
    occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
) -> u32 {
    let kind = HandKind::from_labels(labels, card_lut, occurences_lut);

    labels
        .bytes()
        .rev()
        .map(|l| card_lut[l as usize - SMALLEST_LABEL])
        .enumerate()
        .map(|(i, s)| (s as u32) << (i << 2)) // i * 4
        .sum::<u32>()
        | kind as u32
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u32)]
pub enum HandKind {
    FiveOfAKind = 6 << 5 * 4,
    FourOfAKind = 5 << 5 * 4,
    FullHouse = 4 << 5 * 4,
    ThreeOfAKind = 3 << 5 * 4,
    TwoPairs = 2 << 5 * 4,
    OnePair = 1 << 5 * 4,
    FiveUnique = 0 << 5 * 4,
}

impl HandKind {
    fn from_labels(
        labels: &str,
        card_lut: &[u32; CARD_LUT_LEN],
        occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
    ) -> Self {
        let mut card_counts = 0u64;
        let mut count_occurences = NUM_LABELS as usize;
        let mut num_jokers = 0usize;

        for l in labels.chars() {
            let card = card_lut[l as usize - SMALLEST_LABEL] as usize;

            let card_count = (card_counts >> 4 * card) & 0xf;
            count_occurences -= 1usize << card_count * 4;

            card_counts += 1u64 << card * 4;

            let card_count = (card_counts >> 4 * card) & 0xf;
            count_occurences += 1usize << card_count * 4;

            num_jokers += (l == 'J') as usize;
        }

        occurences_lut[count_occurences_to_lut_idx(count_occurences) + num_jokers]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cmp_hands() {
        assert!(
            labels_to_hand_strength(
                "QQQ23",
                &LABEL_TO_STRENGTH_LUT_0,
                &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0
            ) > labels_to_hand_strength(
                "A3456",
                &LABEL_TO_STRENGTH_LUT_0,
                &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0
            )
        );
        assert!(
            labels_to_hand_strength(
                "QQQ24",
                &LABEL_TO_STRENGTH_LUT_0,
                &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0
            ) > labels_to_hand_strength(
                "QQQ23",
                &LABEL_TO_STRENGTH_LUT_0,
                &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0
            )
        );
    }

    #[test]
    fn test_kind_from_labels() {
        let kind_lut = &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0;

        assert_eq!(
            HandKind::from_labels("QQQ23", &LABEL_TO_STRENGTH_LUT_0, kind_lut),
            HandKind::ThreeOfAKind
        );
    }

    #[test]
    fn test_kind_from_labels_with_jokers() {
        assert_kind_with_jokers("2345J", HandKind::OnePair);
        assert_kind_with_jokers("JJJJJ", HandKind::FiveOfAKind);
        assert_kind_with_jokers("3AJ6J", HandKind::ThreeOfAKind);

        // Five unique
        assert_kind_with_jokers("23456", HandKind::FiveUnique);

        // One pair
        assert_kind_with_jokers("2345J", HandKind::OnePair);
        assert_kind_with_jokers("2345J", HandKind::OnePair);

        // Full house
        assert_kind_with_jokers("JJJQQ", HandKind::FiveOfAKind);
        assert_kind_with_jokers("QQQJJ", HandKind::FiveOfAKind);
        assert_kind_with_jokers("QQQAA", HandKind::FullHouse);
    }

    fn assert_kind_with_jokers(labels: &str, kind: HandKind) {
        assert_eq!(
            HandKind::from_labels(
                labels,
                &LABEL_TO_STRENGTH_LUT_1,
                &COUNT_OCCURENCES_TO_HAND_KIND_LUT_1
            ),
            kind
        );
    }
}
