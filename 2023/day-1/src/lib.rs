use lazy_static::lazy_static;
use regex::Regex;

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, last) = first_and_last_digit(line.as_bytes());
            10 * first + last
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, last) = first_and_last_digit_with_names(line.as_bytes());
            10 * first + last
        })
        .sum()
}

lazy_static! {
    static ref DIGIT_PATTERN: Regex = Regex::new(r"(\d)").unwrap();
}

fn first_and_last_digit(line: &[u8]) -> (u32, u32) {
    let mut first = 0;
    let mut last = 0;
    for c in line.iter() {
        match c {
            b'0'..=b'9' => {
                last = (c - b'0') as u32;
            }
            _ => {}
        }

        if first == 0 {
            first = last;
        }
    }

    (first, last)
}

fn first_and_last_digit_with_names(line: &[u8]) -> (u32, u32) {
    let mut first = -1i32;
    let mut last = -1i32;
    for (i, c) in line.iter().enumerate() {
        match c {
            b'0'..=b'9' => {
                last = (c - b'0') as i32;
            }
            b'e' => {
                if &line[i..(i + 5).min(line.len())] == b"eight" {
                    last = 8;
                }
            }
            b'f' => {
                if &line[i..(i + 4).min(line.len())] == b"five" {
                    last = 5;
                } else if &line[i..(i + 4).min(line.len())] == b"four" {
                    last = 4;
                }
            }
            b'n' => {
                if &line[i..(i + 4).min(line.len())] == b"nine" {
                    last = 9;
                }
            }
            b'o' => {
                if &line[i..(i + 3).min(line.len())] == b"one" {
                    last = 1;
                }
            }
            b's' => {
                if &line[i..(i + 5).min(line.len())] == b"seven" {
                    last = 7;
                } else if &line[i..(i + 3).min(line.len())] == b"six" {
                    last = 6;
                }
            }
            b't' => {
                if &line[i..(i + 3).min(line.len())] == b"two" {
                    last = 2;
                } else if &line[i..(i + 5).min(line.len())] == b"three" {
                    last = 3;
                }
            }
            _ => {}
        }
        if first < 0 {
            first = last;
        }
    }

    (first as u32, last as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_calibration_values() {
        assert_eq!(
            part_2(
                r#"1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet"#
            ),
            142
        );
        assert_eq!(
            part_2(
                r#"two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"#
            ),
            281
        );
    }
}
