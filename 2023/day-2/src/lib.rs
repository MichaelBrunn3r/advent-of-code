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
                    match *data.add("1 ".len()) {
                        b'r' => {
                            data = data.add("1 red".len());
                        }
                        b'g' => {
                            data = data.add("1 green".len());
                        }
                        b'b' => {
                            data = data.add("1 blue".len());
                        }
                        _ => {}
                    }
                } else {
                    let mut amount = *data - b'0';
                    amount = amount * 10 + (*data.offset(1) - b'0');

                    match *data.add("12 ".len()) {
                        b'r' => {
                            if amount > 12 {
                                is_game_valid = false;
                            }
                            data = data.add("12 red".len());
                        }
                        b'g' => {
                            if amount > 13 {
                                is_game_valid = false;
                            }
                            data = data.add("12 green".len());
                        }
                        b'b' => {
                            if amount > 14 {
                                is_game_valid = false;
                            }
                            data = data.add("12 blue".len());
                        }
                        _ => {}
                    }
                }

                match *data {
                    b'\n' => {
                        data = data.add("\n".len());
                        break 'game;
                    }
                    _ => {
                        data = data.add(", ".len());
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
    let mut data = input.as_ptr();
    let mut sum = 0usize;

    for &len in GID_PREFIX_LEN_LUT.iter() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        unsafe {
            // #GidDigits = {1:9, 2:90, 3:1}
            data = data.offset(len);

            'game: loop {
                // #AmountDigits = {1:956, 2:288}
                if *data.offset(1) == b' ' {
                    match *data.add("1 ".len()) {
                        b'r' => {
                            max_red = max_red.max((*data - b'0') as usize);
                            data = data.add("1 red".len());
                        }
                        b'g' => {
                            max_green = max_green.max((*data - b'0') as usize);
                            data = data.add("1 green".len());
                        }
                        b'b' => {
                            max_blue = max_blue.max((*data - b'0') as usize);
                            data = data.add("1 blue".len());
                        }
                        _ => {}
                    }
                } else {
                    let mut amount = *data - b'0';
                    amount = amount * 10 + (*data.offset(1) - b'0');

                    match *data.add("12 ".len()) {
                        b'r' => {
                            max_red = max_red.max(amount as usize);
                            data = data.add("12 red".len());
                        }
                        b'g' => {
                            max_green = max_green.max(amount as usize);
                            data = data.add("12 green".len());
                        }
                        b'b' => {
                            max_blue = max_blue.max(amount as usize);
                            data = data.add("12 blue".len());
                        }
                        _ => {}
                    }
                }

                match *data {
                    b'\n' => {
                        data = data.add("\n".len());
                        break 'game;
                    }
                    _ => {
                        data = data.add(", ".len());
                    }
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }

    sum
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
