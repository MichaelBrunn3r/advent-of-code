const NUM_LINES: usize = 1000;
pub fn part_1(input: &str) -> usize {
    let mut input = input.as_ptr();
    let mut sum = 0;

    unsafe {
        for _ in 0..NUM_LINES {
            while (*input & 0b0100_0000) != 0 {
                input = input.add(1);
            }

            let first = *input;
            let mut last = *input;

            input = input.add(1);

            while *input != b'\n' {
                if (*input & 0b0100_0000) == 0 {
                    last = *input;
                }
                input = input.add(1);
            }

            sum += 10 * (first - b'0') as usize + (last - b'0') as usize;
            input = input.add(1);
        }
    }

    sum
}

const ONE: u32 = 0b01100101_01101110_01101111; // ?eno
const TWO: u32 = 0b01101111_01110111_01110100; // ?owt
const THREE: u32 = 0b01100101_01110010_01101000_01110100; // erht
const FOUR: u32 = 0b01110010_01110101_01101111_01100110; // ruof
const FIVE: u32 = 0b01100101_01110110_01101001_01100110; // evif
const SIX: u32 = 0b01111000_01101001_01110011; // ?xis
const SEVEN: u32 = 0b01100101_01110110_01100101_01110011; // eves
const EIGHT: u32 = 0b01101000_01100111_01101001_01100101; // hgie
const NINE: u32 = 0b01100101_01101110_01101001_01101110; // enin

pub fn part_2(input: &str) -> usize {
    let mut input = input.as_ptr();
    let mut sum = 0;

    unsafe {
        for _ in 0..NUM_LINES {
            let mut first = 0;
            let mut last = 0;

            while *input != b'\n' {
                let val = (input as *const u32).read();
                let three_bytes = val & 0b11111111_11111111_11111111;

                if val & 0b01000000 == 0 {
                    last = *input - b'0';
                    input = input.add(1);
                } else if three_bytes == ONE {
                    last = 1;
                    input = input.add(2);
                } else if three_bytes == TWO {
                    last = 2;
                    input = input.add(2);
                } else if val == THREE {
                    last = 3;
                    input = input.add(3);
                } else if val == FOUR {
                    last = 4;
                    input = input.add(1);
                } else if val == FIVE {
                    last = 5;
                    input = input.add(3);
                } else if three_bytes == SIX {
                    last = 6;
                    input = input.add(3);
                } else if val == SEVEN {
                    last = 7;
                    input = input.add(1);
                } else if val == EIGHT {
                    last = 8;
                    input = input.add(4);
                } else if val == NINE {
                    last = 9;
                    input = input.add(3);
                } else {
                    input = input.add(1);
                }

                if first == 0 {
                    first = last;
                }
            }

            sum += 10 * first as usize + last as usize;
            input = input.add(1);
        }
    }
    sum
}
