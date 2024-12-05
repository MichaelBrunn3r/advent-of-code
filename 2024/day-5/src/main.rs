use aoc_2024_5::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (rules, updates) = parse(&input);
    println!("Part 1: {}", p1(&rules, &updates));
    println!("Part 2: {}", p2(&rules, updates));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
        let (rules, updates) = parse(&input);
        assert_eq!(p1(&rules, &updates), 4959);
    }

    #[test]
    fn test_p2() {
        let input = aoc::read_input_to_string();
        let (rules, updates) = parse(&input); 
        assert_eq!(p2(&rules, updates), 4655);
    }
}