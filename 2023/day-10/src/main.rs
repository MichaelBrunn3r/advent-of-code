use aoc_2023_10::*;

fn main() {
    println!("Part 1: {}", part_1(&mut aoc::read_input_to_string()));
    println!("Part 2: {}", part_2(&mut aoc::read_input_to_string()));
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
        aoc::assert_solution(2, part_2);
    }

    #[test]
    fn test_example_3() {
        aoc::assert_solution(3, part_2);
    }

    #[test]
    fn test_example_4() {
        aoc::assert_solution(4, part_2);
    }

    #[test]
    fn test_example_5() {
        aoc::assert_solution(5, part_2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&mut aoc::read_input_to_string()), 7145);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&mut aoc::read_input_to_string()), 445);
    }
}
