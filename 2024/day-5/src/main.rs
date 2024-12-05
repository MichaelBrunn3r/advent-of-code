use aoc_2024_5::*;

fn main() {
    let input = aoc::read_input_to_string();
    let rules = parse(&input);
    println!("Part 1: {}", p1(&input, &rules));
    println!("Part 2: {}", p2(&input, &rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
    let rules = parse(&input);
        assert_eq!(p1(&input, &rules), 4959);
    }

    #[test]
    fn test_p2() {
        let input = aoc::read_input_to_string();
        let rules = parse(&input);  
        assert_eq!(p2(&input, &rules), 4655);
    }
}