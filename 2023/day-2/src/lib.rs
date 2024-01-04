use parse::GameIterator;

pub mod parse;

pub fn part_1(input: &str) -> usize {
    let possible_games = GameIterator::new(input.as_ptr()).filter(|(_, reveals)| {
        reveals
            .iter()
            .all(|reveal| reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14)
    });
    possible_games.map(|(gid, _)| gid).sum()
}

pub fn part_2(input: &str) -> usize {
    let games = input.split('\n').map(|line| parse_game(line));
    games
        .map(|game| {
            let max_red = game.reveals.iter().map(|reveal| reveal.red).max().unwrap();
            let max_green = game
                .reveals
                .iter()
                .map(|reveal| reveal.green)
                .max()
                .unwrap();
            let max_blue = game.reveals.iter().map(|reveal| reveal.blue).max().unwrap();
            (max_red * max_green * max_blue) as usize
        })
        .sum()
}

fn parse_game(line: &str) -> Game {
    let (header, body) = line.split_once(':').expect("Could not split off reveals");
    let gid = header
        .trim()
        .split_once(' ')
        .expect("Could not split off game id")
        .1
        .parse::<u32>()
        .expect("Could not parse game id");

    let reveals = body.split(';').map(|reaveal| {
        Reveal::from_entries(reaveal.split(',').map(|entry| {
            let (amount, color) = entry
                .trim()
                .split_once(' ')
                .expect("Could not split amount/color");
            (
                amount.parse::<u32>().expect("Could not parse amount"),
                Color::from_str(color).expect("Could not parse color"),
            )
        }))
    });

    Game {
        gid,
        reveals: reveals.collect(),
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "red" => Some(Self::Red),
            "green" => Some(Self::Green),
            "blue" => Some(Self::Blue),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn from_entries(entries: impl Iterator<Item = (u32, Color)>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for (amount, color) in entries {
            match color {
                Color::Red => red += amount,
                Color::Green => green += amount,
                Color::Blue => blue += amount,
            }
        }
        Self { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    gid: u32,
    reveals: Vec<Reveal>,
}
