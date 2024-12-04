use aoc_2023_11::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input, 999_999));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
        assert_eq!(p1(&input), 10173804);
    }

    #[test]
    fn test_p2() {
        let input = aoc::read_input_to_string();
        assert_eq!(p2(&input, 999_999), 634324905172);
    }
}
