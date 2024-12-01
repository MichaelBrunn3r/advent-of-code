use aoc_2023_16::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    // println!("Part 1: {}", part_1(&aoc::read_example_to_string(0)));
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
        aoc::assert_solution(1, part_1);
    }

    #[test]
    fn test_example_2() {
        aoc::assert_solution(2, part_1);
    }

    #[test]
    fn test_example_3() {
        aoc::assert_solution(3, part_1);
    }

    #[test]
    fn test_example_4() {
        aoc::assert_solution(4, part_1);
    }

    #[test]
    fn test_example_5() {
        aoc::assert_solution(5, part_1);
    }
}
