use aoc_2023_5::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (seeds, map_sections) = parse(&input);

    println!("Part 1: {}", part_1(&seeds, &map_sections));
    println!("Part 2: {}", part_2(&seeds, &map_sections));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (seeds, map_sections) = parse(&aoc::read_input_to_string());
        assert_eq!(part_1(&seeds, &map_sections), 388071289);
    }

    #[test]
    fn test_part_2() {
        let (seeds, map_sections) = parse(&aoc::read_input_to_string());
        assert_eq!(part_2(&seeds, &map_sections), 84206669);
    }
}
