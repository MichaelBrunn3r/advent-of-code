use aoc_2024_5::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (rules, correct_updates, wrong_updates) = parse(&input);
    println!("Part 1: {}", p1(&correct_updates));
    println!("Part 2: {}", p2(&rules, wrong_updates));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
        let (_, correct_updates, _) = parse(&input);
        assert_eq!(p1(&correct_updates), 4959);
    }

    #[test]
    fn test_p2() {
        let input = aoc::read_input_to_string();
        let (rules, _, wrong_updates) = parse(&input); 
        assert_eq!(p2(&rules, wrong_updates), 4655);
    }
}