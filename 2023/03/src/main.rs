use std::{fs, io::BufReader, ops::RangeInclusive};

use regex::Regex;

const MAX_PART_NUM_LEN: u8 = 3;

fn main() {
    let input = std::fs::read_to_string("input").expect("file not found");
    let lines = prepare_input(&input);

    println!("Sum of part numbers = {}", sum_part_numbers(&input));
    println!("Sum of gear ratios = {}", sum_gear_ratios(&lines));
}

fn sum_part_numbers(input: &str) -> u32 {
    let re = Regex::new(r"\d+").expect("invalid regex");

    let mut part_numbers: Vec<u32> = Vec::new();

    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let range = m.range();
            let mut is_part_number = false;

            if range.start > 0 {
                let prev_char = line.chars().nth(range.start - 1).unwrap();
                if prev_char != '.' && !prev_char.is_digit(10) {
                    is_part_number = true;
                }
            }

            if !is_part_number && range.end < line.len() {
                let next_char = line.chars().nth(range.end).unwrap();
                if next_char != '.' && !next_char.is_digit(10) {
                    is_part_number = true;
                }
            }

            if !is_part_number && i > 0 {
                let prev_line = lines[i - 1];
                for j in range.start.saturating_sub(1)..(range.end + 1).min(prev_line.len()) {
                    let above_char = prev_line.chars().nth(j).unwrap();
                    if above_char != '.' && !above_char.is_digit(10) {
                        is_part_number = true;
                        break;
                    }
                }
            }

            if !is_part_number && i < lines.len() - 1 {
                let next_line = lines[i + 1];
                for j in range.start.saturating_sub(1)..(range.end + 1).min(next_line.len()) {
                    let below_char = next_line.chars().nth(j).unwrap();
                    if below_char != '.' && !below_char.is_digit(10) {
                        is_part_number = true;
                        break;
                    }
                }
            }

            if is_part_number {
                part_numbers.push(
                    line.get(range)
                        .expect("invalid range")
                        .parse::<u32>()
                        .unwrap(),
                );
            }
        }
    }

    part_numbers.iter().sum()
}

fn sum_gear_ratios(lines: &[Vec<u8>]) -> u32 {
    let mut sum = 0;

    for pos in star_positions(&lines) {
        let numbers = adjacent_numbers(&lines, pos);
        if numbers.len() != 2 {
            continue;
        }

        let a = std::str::from_utf8(numbers[0])
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let b = std::str::from_utf8(numbers[1])
            .unwrap()
            .parse::<u32>()
            .unwrap();
        sum += a * b;
    }

    sum
}

fn adjacent_numbers(lines: &[Vec<u8>], star_position: (usize, usize)) -> Vec<&[u8]> {
    let (star_line, star_pos) = star_position;
    let mut numbers = Vec::new();

    // Find numbers above and below
    for line_idx in [star_line - 1, star_line + 1] {
        let line = &lines[line_idx];

        let mut char_pos = star_pos - 1;
        while char_pos <= star_pos + 1 {
            if !line[char_pos].is_ascii_digit() {
                char_pos += 1;
                continue;
            }

            let mut left = char_pos;
            if char_pos < star_pos {
                while left > 0 && line[left - 1].is_ascii_digit() {
                    left -= 1;
                }
            }
            let mut right = char_pos;
            while right < line.len() - 1 && line[right + 1].is_ascii_digit() {
                right += 1;
            }

            numbers.push(&line[left..=right]);

            char_pos = right + 1;
        }
    }

    // Find number left of star
    let line = &lines[star_line];
    if line[star_pos - 1].is_ascii_digit() {
        let mut left = star_pos - 1;
        while left > 0 && line[left - 1].is_ascii_digit() {
            left -= 1;
        }

        numbers.push(&line[left..=star_pos - 1]);
    }

    // Find number right of star
    if line[star_pos + 1].is_ascii_digit() {
        let mut right = star_pos + 1;
        while right < line.len() - 1 && line[right + 1].is_ascii_digit() {
            right += 1;
        }

        numbers.push(&line[star_pos + 1..=right]);
    }

    numbers
}

fn star_positions<'l>(lines: &'l [Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + 'l {
    lines.iter().enumerate().flat_map(|(l, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, char)| **char == b'*')
            .map(move |(c, _)| (l, c))
    })
}

fn sum_gear_ratios_2(input: &str) -> u32 {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut sum = 0;

    let re_star = Regex::new(r"\*").expect("invalid regex");
    let re_num = Regex::new(r"\d+").expect("invalid regex");
    for (i, line) in lines.iter().enumerate() {
        for m_star in re_star.find_iter(&line) {
            let range_star = m_star.range();
            let mut num_matches: Vec<regex::Match<'_>> = Vec::new();

            // Above
            if i > 0 {
                let range_above = range_star.start.saturating_sub(3)
                    ..(range_star.end + 3).min(lines[i - 1].len());
                for m_num in re_num.find_iter(&lines[i - 1][range_above.clone()]) {
                    let mut range_num = m_num.range();
                    range_num.start += range_above.start;
                    range_num.end += range_above.start;
                    if range_num.start == range_star.start.saturating_sub(1)
                        || range_num.start == range_star.start
                        || range_num.start == range_star.start + 1
                        || range_num.end == range_star.end.saturating_sub(1)
                        || range_num.end == range_star.end
                        || range_num.end == range_star.end + 1
                    {
                        num_matches.push(m_num);
                    }
                }
            }

            // Horizontal
            let range_horizontal =
                range_star.start.saturating_sub(3)..(range_star.end + 3).min(line.len());
            for m_num in re_num.find_iter(&line[range_horizontal.clone()]) {
                let mut range_num = m_num.range();
                range_num.start += range_horizontal.start;
                range_num.end += range_horizontal.start;
                if range_num.start == range_star.start.saturating_sub(1)
                    || range_num.start == range_star.start
                    || range_num.start == range_star.start + 1
                    || range_num.end == range_star.end.saturating_sub(1)
                    || range_num.end == range_star.end
                    || range_num.end == range_star.end + 1
                {
                    num_matches.push(m_num);
                }
            }

            // Below
            if i < lines.len() - 1 {
                let range_below = range_star.start.saturating_sub(3)
                    ..(range_star.end + 3).min(lines[i + 1].len());
                for m_num in re_num.find_iter(&lines[i + 1][range_below.clone()]) {
                    let mut range_num = m_num.range();
                    range_num.start += range_below.start;
                    range_num.end += range_below.start;
                    if range_num.start == range_star.start.saturating_sub(1)
                        || range_num.start == range_star.start
                        || range_num.start == range_star.start + 1
                        || range_num.end == range_star.end.saturating_sub(1)
                        || range_num.end == range_star.end
                        || range_num.end == range_star.end + 1
                    {
                        num_matches.push(m_num);
                    }
                }
            }

            if num_matches.len() != 2 {
                continue;
            }

            let a = num_matches[0].as_str().parse::<u32>().unwrap();
            let b = num_matches[1].as_str().parse::<u32>().unwrap();
            sum += a * b;
        }
    }

    sum
}

fn prepare_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_numbers() {
        assert_eq!(
            sum_part_numbers(
                &r"467..114..
                  ...*......
                  ..35..633.
                  ......#...
                  617*......
                  .....+.58.
                  ..592.....
                  ......755.
                  ...$.*....
                  .664.598.."
                    .lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .collect::<Vec<&str>>()
                    .join("\n")
            ),
            4361
        );
    }

    #[test]
    fn test_star_positions() {
        assert_eq!(
            star_positions(&prepare_input(
                r"467..114..
                  ...*......
                  ..35..633.
                  ......#...
                  617*......
                  .....+.58.
                  ..592.....
                  ......755.
                  ...$.*....
                  .664.598.."
            ))
            .collect::<Vec<(usize, usize)>>(),
            vec![(1, 3), (4, 3), (8, 5)]
        );
    }

    #[test]
    fn test_adjacent_numbers() {
        assert_eq!(
            adjacent_numbers(
                &prepare_input(
                    r"467.114...
                      ...*12....
                      ..35..633."
                ),
                (1, 3)
            ),
            // 467,
            vec![b"467" as &[u8], b"114", b"35", b"12"]
        );
    }

    #[test]
    fn test_sum_gear_ratios() {
        assert_eq!(
            sum_gear_ratios(&prepare_input(
                r"467..114..
                  ...*......
                  ..35..633.
                  ......#...
                  617*......
                  .....+.58.
                  ..592.....
                  ......755.
                  ...$.*....
                  .664.598.."
            )),
            467835
        );

        assert_eq!(
            // 992 * 806 + 405 * 67 + 819 * 478 + 196 * 313 + 675*861 + 276 * 155 + 692 * 985 + 207 * 160 + 80 * 31 + 938 * 233 + 75 * 997 + 285 * 521 + 181 * 606 + 946 * 437
            sum_gear_ratios(&prepare_input(
                r"............................................................................................................................................
                  ........405...819.........514..............201....*....*806.....196......*........*............../...........@..................644....*195.
                  ........*......*.................@.....276......538.992...........*....720.692..880........+117.266..207.........+..........................
                  ........67....478..675*861...80..34.....*..+777..................313........*.......................*.........445.........200..*...@........
                  ..938......75..................*.....155.................................985..#........285.....181...160.....................$.872..595.....
                  ....*..997*....................31.............148......946...........803.......195.......*.944...*.......551+........*...867................
                  ...233..........553.596...........436..........*..........*437..559-..*.............@..521.*......606..........519.226..........@...........
                  ............................................................................................................................................"
            )),
            3585594
        );
    }
}
