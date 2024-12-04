use aoc_2024_3::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_input_to_string()), 184122457);
    }

    #[test]
    fn test_p1_regex() {
        assert_eq!(p1_regex(&aoc::read_input_to_string()), 184122457);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_input_to_string()), 107862689);
    }

    #[test]
    fn test_p2_regex() {
        assert_eq!(p2_regex(&aoc::read_input_to_string()), 107862689);
    }
}