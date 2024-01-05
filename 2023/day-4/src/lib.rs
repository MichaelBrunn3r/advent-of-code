const NUM_CARDS: usize = 202;
static mut WINNING_NUMBERS: [[bool; 14650]; NUM_CARDS] = unsafe { std::mem::zeroed() };

pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    let mut total = 0;

    unsafe {
        for cid in 0..NUM_CARDS {
            data = data.offset("Card   1: ".len() as isize);

            for _ in 0..10 {
                let num = (data as *const u16).read();
                WINNING_NUMBERS[cid][num as usize] = true;
                data = data.offset("12 ".len() as isize);
            }

            data = data.offset("| ".len() as isize);

            let mut points = 1;
            for _ in 0..25 {
                let num = (data as *const u16).read();
                points <<= WINNING_NUMBERS[cid][num as usize] as usize;
                data = data.offset("12 ".len() as isize);
            }
            points >>= 1;

            total += points;
        }
    }

    total
}

pub fn part_2(input: &str) -> usize {
    let mut matches_per_card: [usize; NUM_CARDS] = unsafe { std::mem::zeroed() };

    let mut data = input.as_ptr();
    unsafe {
        for cid in 0..NUM_CARDS {
            data = data.offset("Card   1: ".len() as isize);

            for _ in 0..10 {
                let num = (data as *const u16).read();
                WINNING_NUMBERS[cid][num as usize] = true;
                data = data.offset("12 ".len() as isize);
            }

            data = data.offset("| ".len() as isize);

            let mut num_matches = 0;
            for _ in 0..25 {
                let num = (data as *const u16).read();
                num_matches += WINNING_NUMBERS[cid][num as usize] as usize;
                data = data.offset("12 ".len() as isize);
            }

            matches_per_card[cid] = num_matches;
        }
    }

    let mut cards = (0..NUM_CARDS).collect::<Vec<_>>();
    let mut total_cards = NUM_CARDS;

    while !cards.is_empty() {
        let card_id = cards.pop().unwrap();
        let num_matches = matches_per_card[card_id];
        total_cards += num_matches;

        for won_id in card_id + 1..card_id + 1 + num_matches {
            cards.push(won_id);
        }
    }

    total_cards
}
