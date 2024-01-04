pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    let mut sum = 0;

    for gid in 1..=100 {
        let mut is_game_valid = true;

        unsafe {
            // #GidDigits = {1:9, 2:90, 3:1}
            if gid < 10 {
                data = data.offset(8);
            } else if gid < 100 {
                data = data.offset(9);
            } else {
                data = data.offset(10);
            }

            'reveals: loop {
                'sets: for _ in 0..3 {
                    // #AmountDigits = {1:956, 2:288}
                    let mut amount = *data - b'0';
                    data = data.offset(1);
                    if *data != b' ' {
                        amount = amount * 10 + (*data - b'0');
                        data = data.offset(1);
                    }

                    if *data.offset(" red".len() as isize) < b'a' {
                        if amount > 12 {
                            is_game_valid = false;
                        }
                        data = data.offset(" red".len() as isize);
                    } else if *data.offset(" blue".len() as isize) < b'a' {
                        if amount > 14 {
                            is_game_valid = false;
                        }
                        data = data.offset(" blue".len() as isize);
                    } else {
                        if amount > 13 {
                            is_game_valid = false;
                        }
                        data = data.offset(" green".len() as isize);
                    };

                    match *data {
                        b';' => {
                            data = data.offset("; ".len() as isize);
                            break 'sets;
                        }
                        b',' => {
                            data = data.offset(", ".len() as isize);
                        }
                        _ => {
                            data = data.offset("\n".len() as isize);
                            break 'reveals;
                        }
                    }
                }
            }
        }
        if is_game_valid {
            sum += gid as usize;
        }
    }

    sum
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
