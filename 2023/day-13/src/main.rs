use aoc_2023_13::*;

fn main() {
    let input = aoc::read_input_to_string();
    let patterns = parse(&input);

    println!("Part 1: {}", part_1(&patterns));
    println!("Part 2: {}", part_2(&patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = aoc::read_input_to_string();
        assert_eq!(part_1(&parse(&input)), 33728);
    }

    #[test]
    fn test_part_2() {
        let input = aoc::read_input_to_string();
        assert_eq!(part_2(&parse(&input)), 28235);
    }
}
