use aoc_2023_12::*;

fn main() {
    let input = aoc::read_input_to_string();
    // println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
    println!("Part 2: {}", part_2(&aoc::read_example_to_string(0)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, part_2);
    }

    // #[test]
    // fn test_part_1() {
    //     let input = aoc::read_input_to_string();
    //     assert_eq!(part_1(&input), 7090);
    // }
}
