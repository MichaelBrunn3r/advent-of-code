pub mod lut;

use aoc::U8PtrExt;
use lut::*;

const NUM_LINES: usize = 1000;
pub const CARD_LUT_LEN: usize = 36;
pub const SMALLEST_LABEL: usize = b'2' as usize;
const NUM_LABELS: usize = 13;

pub fn part_1(input: &str) -> usize {
    count_winnings(
        input,
        &LABEL_TO_STRENGTH_LUT_0,
        &COUNT_OCCURENCES_TO_HAND_KIND_LUT_0,
    )
}

pub fn part_2(input: &str) -> usize {
    count_winnings(
        input,
        &LABEL_TO_STRENGTH_LUT_1,
        &COUNT_OCCURENCES_TO_HAND_KIND_LUT_1,
    )
}

fn count_winnings(
    input: &str,
    card_lut: &[u32; CARD_LUT_LEN],
    occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
) -> usize {
    let mut data = input.as_ptr();
    unsafe {
        let mut hands = Vec::with_capacity(NUM_LINES);

        for _ in 0..NUM_LINES {
            let hand_strength = labels_to_hand_strength(
                std::slice::from_raw_parts(data, 5),
                card_lut,
                occurences_lut,
            );
            data = data.add("992QQ ".len());

            let bet = data.parse_uint_n_digits(get_num_bet_digits(data));

            data = data.add(1);

            hands.push((hand_strength, bet));
        }

        hands.sort_by_cached_key(|(hand_strength, _)| *hand_strength);
        hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_, bet))| acc + (i + 1) * bet)
    }
}

unsafe fn get_num_bet_digits(data: *const u8) -> usize {
    // #digits: {1:9, 2:90, 3:900, 4:1}
    if data.add(3).read() == b'\n' {
        3
    } else if data.add(2).read() == b'\n' {
        2
    } else if data.add(1).read() == b'\n' {
        1
    } else {
        4
    }
}

fn labels_to_hand_strength(
    labels: &[u8],
    card_lut: &[u32; CARD_LUT_LEN],
    occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
) -> u32 {
    let kind = HandKind::from_labels(labels, card_lut, occurences_lut);

    labels
        .iter()
        .rev()
        .map(|&l| card_lut[l as usize - SMALLEST_LABEL])
        .enumerate()
        .map(|(i, s)| s << (i << 2)) // i * 4
        .sum::<u32>()
        | kind as u32
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u32)]
pub enum HandKind {
    FiveOfAKind = 6 << (5 * 4),
    FourOfAKind = 5 << (5 * 4),
    FullHouse = 4 << (5 * 4),
    ThreeOfAKind = 3 << (5 * 4),
    TwoPairs = 2 << (5 * 4),
    OnePair = 1 << (5 * 4),
    FiveUnique = 0 << (5 * 4),
}

impl HandKind {
    fn from_labels(
        labels: &[u8],
        card_lut: &[u32; CARD_LUT_LEN],
        occurences_lut: &[HandKind; OCCURENCES_LUT_LEN],
    ) -> Self {
        let mut card_counts = 0u64;
        let mut count_occurences = NUM_LABELS;
        let mut num_jokers = 0usize;

        for &l in labels {
            let card = card_lut[l as usize - SMALLEST_LABEL] as usize;

            let card_count = (card_counts >> (4 * card)) & 0xf;
            count_occurences -= 1usize << (card_count * 4);

            card_counts += 1u64 << (card * 4);

            let card_count = (card_counts >> (4 * card)) & 0xf;
            count_occurences += 1usize << (card_count * 4);

            num_jokers += (l == b'J') as usize;
        }

        occurences_lut[count_occurences_to_lut_idx(count_occurences) + num_jokers]
    }
}
