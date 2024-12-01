const NUM_CARDS: usize = 202;
static mut WINNING_NUMBERS: [[bool; 14650]; NUM_CARDS] = unsafe { std::mem::zeroed() };

pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    let mut total = 0;

    unsafe {
        WINNING_NUMBERS
            .iter_mut()
            .take(NUM_CARDS)
            .for_each(|winnining_numbers| {
                data = data.add("Card   1: ".len());

                for _ in 0..10 {
                    let num = (data as *const u16).read();
                    winnining_numbers[num as usize] = true;
                    data = data.add("12 ".len());
                }

                data = data.add("| ".len());

                let mut points = 1;
                for _ in 0..25 {
                    let num = (data as *const u16).read();
                    points <<= winnining_numbers[num as usize] as usize;
                    data = data.add("12 ".len());
                }
                points >>= 1;

                total += points;
            });
    }

    total
}

static mut MATCHES_PER_CARD: [usize; NUM_CARDS] = unsafe { std::mem::zeroed() };
pub fn part_2(input: &str) -> usize {
    let mut memo: [usize; NUM_CARDS] = unsafe { std::mem::zeroed() };

    let mut data = input.as_ptr();
    unsafe {
        for cid in 0..NUM_CARDS {
            data = data.add("Card   1: ".len());

            for _ in 0..10 {
                let num = (data as *const u16).read();
                WINNING_NUMBERS[cid][num as usize] = true;
                data = data.add("12 ".len());
            }

            data = data.add("| ".len());

            let mut num_matches = 0;
            for _ in 0..25 {
                let num = (data as *const u16).read();
                num_matches += WINNING_NUMBERS[cid][num as usize] as usize;
                data = data.add("12 ".len());
            }

            MATCHES_PER_CARD[cid] = num_matches;
        }
    }

    NUM_CARDS
        + (0..NUM_CARDS)
            .map(|card_id| calc_cards_won_by(card_id, &mut memo))
            .sum::<usize>()
}

fn calc_cards_won_by(card_id: usize, memo: &mut [usize; NUM_CARDS]) -> usize {
    if memo[card_id] != 0 {
        return memo[card_id];
    }

    let num_matches = unsafe { MATCHES_PER_CARD[card_id] };
    let mut cards_won = num_matches;
    for won_id in card_id + 1..card_id + 1 + num_matches {
        cards_won += calc_cards_won_by(won_id, memo);
    }

    memo[card_id] = cards_won;

    cards_won
}
