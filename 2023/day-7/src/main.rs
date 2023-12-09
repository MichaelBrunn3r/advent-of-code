use aoc_2023_7::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
        aoc::assert_solution(1, part_2);
    }

    #[test]
    fn test_input() {
        let input = aoc::read_input_to_string();
        assert_eq!(part_1(&input), 252295678);
        assert_eq!(part_2(&input), 250577259);
    }
}
