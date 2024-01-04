pub struct GameIterator {
    data: *const u8,
    gid: usize,
}

impl GameIterator {
    pub fn new(data: *const u8) -> Self {
        Self { data, gid: 0 }
    }
}

impl Iterator for GameIterator {
    type Item = (usize, Vec<Reveal>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.gid >= 100 {
            return None;
        }
        self.gid += 1;

        unsafe {
            // #GidDigits = {1:9, 2:90, 3:1}
            if self.gid < 10 {
                self.data = self.data.offset(8);
            } else if self.gid < 100 {
                self.data = self.data.offset(9);
            } else {
                self.data = self.data.offset(10);
            }

            let mut reveals = Vec::new();
            'reveals: loop {
                let mut reveal = Reveal::new();
                'sets: for _ in 0..3 {
                    // #AmountDigits = {1:956, 2:288}
                    let mut amount = *self.data - b'0';
                    // println!("char: {}", *self.data as char);
                    self.data = self.data.offset(1);
                    if *self.data != b' ' {
                        // println!("amount: {}", amount);
                        amount = amount * 10 + (*self.data - b'0');
                        self.data = self.data.offset(1);
                    }

                    if *self.data.offset(" red".len() as isize) < b'a' {
                        reveal.red = amount;
                        self.data = self.data.offset(" red".len() as isize);
                    } else if *self.data.offset(" blue".len() as isize) < b'a' {
                        reveal.blue = amount;
                        self.data = self.data.offset(" blue".len() as isize);
                    } else {
                        reveal.green = amount;
                        self.data = self.data.offset(" green".len() as isize);
                    };

                    match *self.data {
                        b';' => {
                            self.data = self.data.offset("; ".len() as isize);
                            break 'sets;
                        }
                        b',' => {
                            self.data = self.data.offset(", ".len() as isize);
                        }
                        _ => {
                            reveals.push(reveal);
                            self.data = self.data.offset("\n".len() as isize);
                            break 'reveals;
                        }
                    }
                }
                reveals.push(reveal);
            }
            return Some((self.gid, reveals));
        }
    }
}

#[derive(Debug)]
pub struct Reveal {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Reveal {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
