pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    let mut sum = 0;

    for (gid, &len) in (1..=100).zip(GID_PREFIX_LEN_LUT.iter()) {
        let mut is_game_valid = true;

        unsafe {
            // #GidDigits = {1:9, 2:90, 3:1}
            data = data.offset(len);

            'game: loop {
                // #AmountDigits = {1:956, 2:288}
                if *data.offset(1) == b' ' {
                    match *data.offset("1 ".len() as isize) {
                        b'r' => {
                            data = data.offset("1 red".len() as isize);
                        }
                        b'g' => {
                            data = data.offset("1 green".len() as isize);
                        }
                        b'b' => {
                            data = data.offset("1 blue".len() as isize);
                        }
                        _ => {}
                    }
                } else {
                    let mut amount = *data - b'0';
                    amount = amount * 10 + (*data.offset(1) - b'0');

                    match *data.offset("12 ".len() as isize) {
                        b'r' => {
                            if amount > 12 {
                                is_game_valid = false;
                            }
                            data = data.offset("12 red".len() as isize);
                        }
                        b'g' => {
                            if amount > 13 {
                                is_game_valid = false;
                            }
                            data = data.offset("12 green".len() as isize);
                        }
                        b'b' => {
                            if amount > 14 {
                                is_game_valid = false;
                            }
                            data = data.offset("12 blue".len() as isize);
                        }
                        _ => {}
                    }
                }

                match *data {
                    b'\n' => {
                        data = data.offset("\n".len() as isize);
                        break 'game;
                    }
                    _ => {
                        data = data.offset(", ".len() as isize);
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
    // let games = input.split('\n').map(|line| parse_game(line));
    // games
    //     .map(|game| {
    //         let max_red = game.reveals.iter().map(|reveal| reveal.red).max().unwrap();
    //         let max_green = game
    //             .reveals
    //             .iter()
    //             .map(|reveal| reveal.green)
    //             .max()
    //             .unwrap();
    //         let max_blue = game.reveals.iter().map(|reveal| reveal.blue).max().unwrap();
    //         (max_red * max_green * max_blue) as usize
    //     })
    //     .sum()
    0
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

const GID_PREFIX_LEN_LUT: [isize; 100] = generate_gid_prefix_len_lut();
const fn generate_gid_prefix_len_lut() -> [isize; 100] {
    let mut gid_prefix_len_lut = [0; 100];

    let mut i = 0;
    while i < 9 {
        gid_prefix_len_lut[i] = "Game 1: ".len() as isize;
        i += 1;
    }

    while i < 99 {
        gid_prefix_len_lut[i] = "Game 10: ".len() as isize;
        i += 1;
    }

    gid_prefix_len_lut[99] = "Game 100: ".len() as isize;

    gid_prefix_len_lut
}
