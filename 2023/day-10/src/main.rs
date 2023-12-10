use aoc_2023_10::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));

    // redraw(input.as_str());
}

pub fn redraw(input: &str) {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => '│',
                    '-' => '─',
                    'L' => '└',
                    'J' => '┘',
                    '7' => '┐',
                    'F' => '┌',
                    _ => c,
                })
                .collect::<String>()
        })
        .for_each(|line| println!("{}", line));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution(0, part_1);
    }

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, part_1);
    }

    #[test]
    fn test_example_2() {
        aoc::assert_solution(1, part_1);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(aoc::read_input_to_string().as_str()), 7145);
    }
}
