use aoc_2023_11::*;

fn main() {
    println!("Part 1: {}", part_1(aoc::read_input_to_string()));
    println!("Part 2: {}", part_2(aoc::read_input_to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution(0, part_1);
    }

    // #[test]
    // fn test_example_1() {
    //     aoc::assert_solution(1, part_2);
    // }
}
