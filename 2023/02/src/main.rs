fn main() {
    let input = std::fs::read_to_string("input").expect("Unable to read file");
    // println!("{}", sum_ids_possible_games(&input));
    println!("{}", sum_of_power_of_minimum_set_of_cubes(&input));
}

fn sum_ids_possible_games(input: &str) -> u32 {
    let games = input.split('\n').map(|line| parse_game(line));
    let possible_games = games.filter(|game| {
        game.reveals
            .iter()
            .all(|reveal| reveal.red <= 12 && reveal.green <= 13 && reveal.blue <= 14)
    });
    possible_games.map(|game| game.gid).sum::<u32>()
}

fn sum_of_power_of_minimum_set_of_cubes(input: &str) -> u32 {
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
            max_red * max_green * max_blue
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_ids_possible_games() {
        assert_eq!(
            sum_ids_possible_games(
                r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            ),
            8
        );
    }

    #[test]
    fn test_sum_of_power_of_minimum_set_of_cubes() {
        assert_eq!(
            sum_of_power_of_minimum_set_of_cubes(
                r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            ),
            2286
        );
    }
}
