use aoc_2023_10::*;

fn main() {
    println!("Part 1: {}", part_1(&mut aoc::read_input_to_string()));
    println!("Part 2: {}", part_2(&mut aoc::read_input_to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&mut aoc::read_input_to_string()), 7145);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&mut aoc::read_input_to_string()), 445);
    }
}
