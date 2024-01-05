static mut WINNING_NUMBERS: [[bool; 14650]; 203] = unsafe { std::mem::zeroed() };

pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    let mut total = 0;

    unsafe {
        for cid in 1..=202 {
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
    let matches_per_card: Vec<usize> = input
        .lines()
        .map(|line| Card::from_str(line))
        .map(|card| card.count_matching())
        .collect();
    let mut card_counts = vec![1; matches_per_card.len()];

    let mut total_cards = matches_per_card.len();
    loop {
        let mut cards_won = 0;
        for (i, num_matches) in matches_per_card.iter().enumerate() {
            if card_counts[i] == 0 {
                continue;
            }
            card_counts[i] -= 1;
            for j in i + 1..=i + num_matches {
                card_counts[j] += 1;
                cards_won += 1;
                total_cards += 1;
            }
        }
        if cards_won == 0 {
            break;
        }
    }

    total_cards
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    your_numbers: Vec<usize>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let (_, body) = input.split_once(':').unwrap();
        let (winning_str, your_str) = body.split_once('|').unwrap();

        let winning_numbers = winning_str
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let your_numbers = your_str
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            winning_numbers,
            your_numbers,
        }
    }

    fn count_matching(&self) -> usize {
        let mut matches = 0;
        for number in self.your_numbers.iter() {
            if self.winning_numbers.contains(number) {
                matches += 1;
            }
        }
        matches
    }
}
