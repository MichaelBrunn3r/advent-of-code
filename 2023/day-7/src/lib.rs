use itertools::Itertools;

const CARD_LUT_LEN: usize = 36;
const SMALLEST_LABEL: usize = b'2' as usize;
const NUM_LABELS: usize = 13;
const LABEL_TO_STRENGTH_LUT_0: [u32; CARD_LUT_LEN] =
    generate_label_to_strength_lut(b"23456789TJQKA");
const LABEL_TO_STRENGTH_LUT_1: [u32; CARD_LUT_LEN] =
    generate_label_to_strength_lut(b"J23456789TQKA");

const OCCURENCES_LUT_LEN: usize = (16 << 3) + 5 + 1;
const COUNT_OCCURENCES_TO_HAND_KIND_LUT_0: [HandKind; OCCURENCES_LUT_LEN] =
    generate_count_occurences_to_hand_kind_lut(false);
const COUNT_OCCURENCES_TO_HAND_KIND_LUT_1: [HandKind; OCCURENCES_LUT_LEN] =
    generate_count_occurences_to_hand_kind_lut(true);

pub fn task_0(input: &str) -> usize {
    count_winnings(
        input,
        &LABEL_TO_STRENGTH_LUT_0,
        &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0,
    ) as usize
}

pub fn task_1(input: &str) -> usize {
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
        let mut joker_count = 0usize;

        for l in labels.chars() {
            let card = card_lut[l as usize - SMALLEST_LABEL] as usize;

            let card_count = (card_counts >> 4 * card) & 0xf;
            count_occurences -= 1usize << card_count * 4;

            card_counts += 1u64 << card * 4;

            let card_count = (card_counts >> 4 * card) & 0xf;
            count_occurences += 1usize << card_count * 4;

            joker_count += (l == 'J') as usize;
        }

        count_occurences_to_hand_kind(count_occurences, joker_count, occurences_lut)
    }
}

const fn generate_label_to_strength_lut(labels: &[u8]) -> [u32; CARD_LUT_LEN] {
    let mut map = [u32::MAX; CARD_LUT_LEN];

    let mut i = 0;
    while i < labels.len() {
        map[labels[i] as usize - SMALLEST_LABEL] = i as u32;
        i += 1;
    }

    map
}

const fn generate_count_occurences_to_hand_kind_lut(
    joker_is_wildcard: bool,
) -> [HandKind; OCCURENCES_LUT_LEN] {
    let possible_count_occurences: [(u32, HandKind); 7] = [
        (0x10000c, HandKind::FiveOfAKind),  // idx=16
        (0x01001b, HandKind::FourOfAKind),  // idx=8
        (0x00110b, HandKind::FullHouse),    // idx=5
        (0x00102a, HandKind::ThreeOfAKind), // idx=4
        (0x00021a, HandKind::TwoPairs),     // idx=2
        (0x000139, HandKind::OnePair),      // idx=1
        (0x000058, HandKind::FiveUnique),   // idx=0
    ];
    let mut lut = [HandKind::FiveUnique; OCCURENCES_LUT_LEN];

    let mut i = 0;
    while i < possible_count_occurences.len() {
        let (occurences, kind) = possible_count_occurences[i];

        let mut num_jokers = 0;
        while num_jokers <= 5 {
            let mut idx = count_occurences_to_unique_idx(occurences as usize) as usize;
            idx = (idx << 3) + num_jokers;

            let mut kind = kind;

            if joker_is_wildcard {
                match kind {
                    HandKind::FourOfAKind => {
                        if num_jokers > 0 {
                            kind = HandKind::FiveOfAKind;
                        }
                    }
                    HandKind::FullHouse => {
                        if num_jokers > 0 {
                            kind = HandKind::FiveOfAKind;
                        }
                    }
                    HandKind::ThreeOfAKind => {
                        if num_jokers > 0 {
                            kind = HandKind::FourOfAKind;
                        }
                    }
                    HandKind::TwoPairs => {
                        if num_jokers == 1 {
                            kind = HandKind::FullHouse;
                        } else if num_jokers == 2 {
                            kind = HandKind::FourOfAKind;
                        }
                    }
                    HandKind::OnePair => {
                        if num_jokers == 1 {
                            kind = HandKind::ThreeOfAKind;
                        } else if num_jokers == 2 {
                            kind = HandKind::ThreeOfAKind;
                        }
                    }
                    HandKind::FiveUnique => {
                        if num_jokers == 1 {
                            kind = HandKind::OnePair;
                        }
                    }
                    _ => {}
                }
            }

            lut[idx as usize] = kind;

            num_jokers += 1;
        }

        i += 1;
    }

    lut
}

const fn count_occurences_to_unique_idx(mut occurences: usize) -> usize {
    occurences >>= 8;
    (occurences & 0xf)
        + ((occurences & 0xf0) >> 2)
        + ((occurences & 0xf00) >> 5)
        + ((occurences & 0xf000) >> 8)
}

fn count_occurences_to_hand_kind(
    occurences: usize,
    num_jokers: usize,
    occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
) -> HandKind {
    let mut idx = count_occurences_to_unique_idx(occurences) as usize;
    idx = (idx << 3) + num_jokers;
    occurences_lut[idx]
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
