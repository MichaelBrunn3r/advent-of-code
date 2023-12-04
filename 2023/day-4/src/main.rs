use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT_DIR: PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
}

fn main() {
    let input = std::fs::read_to_string(PROJECT_DIR.join("input")).unwrap();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    input
        .lines()
        .map(|line| Card::from_str(line))
        // .inspect(|card| println!("{:?}", card))
        .map(|card| card.count_matching())
        // .inspect(|matches| println!("#matches={}", matches))
        .filter(|matches| *matches > 0)
        .map(|matches| 2usize.pow((matches - 1) as u32))
        // .inspect(|matches| println!("#points={}", matches))
        .sum::<usize>()
}

fn task_1(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_0() {
        let input = std::fs::read_to_string(PROJECT_DIR.join("example_0")).unwrap();
        let expected = std::fs::read_to_string(PROJECT_DIR.join("solution_0"))
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(task_0(&input), expected);
    }

    #[test]
    fn test_task_1() {
        let input = std::fs::read_to_string(PROJECT_DIR.join("example_1")).unwrap();
        println!("input={}", input);
        let expected = std::fs::read_to_string(PROJECT_DIR.join("solution_1"))
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(task_1(&input), expected);
    }
}
