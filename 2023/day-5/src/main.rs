use aoc_2023_5::*;

fn main() {
    let input = aoc::read_input_to_string();
    let input = parse(&input);

    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let (seeds, map_sections) = parse(&aoc::read_input_to_string());
        assert_eq!(p1(&seeds, &map_sections), 388071289);
    }

    #[test]
    fn test_p2() {
        let (seeds, map_sections) = parse(&aoc::read_input_to_string());
        assert_eq!(p2(&seeds, &map_sections), 84206669);
    }
}
