use aoc_2024_2::*;

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
        assert_eq!(part_1(&aoc::read_input_to_string()), 591);
    }

    // #[test]
    // fn test_part_2() {
    //     parse(&aoc::read_input_to_string(), &mut left, &mut right);
    //     assert_eq!(part_2(&mut left, &mut right), 0);
    // }
}