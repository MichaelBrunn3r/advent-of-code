use aoc_2023_19::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (workflows, rules, parts) = parse(&input);

    println!("Part 1: {}", part_1(workflows, rules, parts));
    println!("Part 2: {}", part_2(rules, workflows));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parts() {
        let input = aoc::read_input_to_string();
        let (workflows, rules, parts) = parse(&input);

        assert_eq!(part_1(workflows, rules, parts), 418498);
        assert_eq!(part_2(rules, workflows), 123331556462603)
    }
}
