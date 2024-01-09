use aoc_2023_9::*;

fn main() {
    let input = aoc::read_input_to_string();
    let data = parse(&input);

    println!("Part 1: {}", part_1(data));
    // println!("Part 2: {}", part_2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(parse(&aoc::read_input_to_string())), 1637452029);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(parse(&aoc::read_input_to_string())), 908);
    }
}
