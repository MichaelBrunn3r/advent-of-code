use aoc_2023_12::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = aoc::read_input_to_string();
        assert_eq!(part_1(&input), 7090);
    }

    #[test]
    fn test_part_2() {
        let input = aoc::read_input_to_string();
        assert_eq!(part_2(&input), 6792010726878);
    }
}
