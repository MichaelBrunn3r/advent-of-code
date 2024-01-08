use aoc_2023_24::*;

fn main() {
    let input = aoc::read_input_to_string();
    let hailstones = parse(&input);
    println!("Part 1: {}", part_1(hailstones));
    println!("Part 2: {}", part_2(hailstones));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(parse(&aoc::read_input_to_string())), 20434);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&aoc::read_input_to_string()), 253302889093151);
    // }
}
