use crate::*;

pub const LABEL_TO_STRENGTH_LUT_0: [u32; CARD_LUT_LEN] =
    generate_label_to_strength_lut(b"23456789TJQKA");
pub const LABEL_TO_STRENGTH_LUT_1: [u32; CARD_LUT_LEN] =
    generate_label_to_strength_lut(b"J23456789TQKA");

const fn generate_label_to_strength_lut(labels: &[u8]) -> [u32; CARD_LUT_LEN] {
    let mut map = [u32::MAX; CARD_LUT_LEN];

    let mut i = 0;
    while i < labels.len() {
        map[labels[i] as usize - SMALLEST_LABEL] = i as u32;
        i += 1;
    }

    map
}

pub const OCCURENCES_LUT_LEN: usize = (16 << 3) + 5 + 1;
pub const COUNT_OCCURENCES_TO_HAND_KIND_LUT_0: [HandKind; OCCURENCES_LUT_LEN] =
    generate_count_occurences_to_hand_kind_lut(false);
pub const COUNT_OCCURENCES_TO_HAND_KIND_LUT_1: [HandKind; OCCURENCES_LUT_LEN] =
    generate_count_occurences_to_hand_kind_lut(true);

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
            let idx = count_occurences_to_lut_idx(occurences as usize) + num_jokers;
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
                        if num_jokers == 1 || num_jokers == 2 {
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

            lut[idx] = kind;

            num_jokers += 1;
        }

        i += 1;
    }

    lut
}

pub const fn count_occurences_to_lut_idx(occurences: usize) -> usize {
    let mut idx = occurences >> 8;
    idx = (idx & 0xf) + ((idx & 0xf0) >> 2) + ((idx & 0xf00) >> 5) + ((idx & 0xf000) >> 8);
    idx << 3
}
