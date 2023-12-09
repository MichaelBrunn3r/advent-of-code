use aoc_2023_9::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Task 2: {}", part_2(&input));
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
    fn test_part_1() {
        assert_eq!(part_1(&aoc::read_input_to_string()), 1637452029);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&aoc::read_input_to_string()), 908);
    }
}
