use aoc_2023_14::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&mut aoc::read_input_to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
        assert_eq!(p1(&input), 107430);
    }

    #[test]
    fn test_p2() {
        let mut input = aoc::read_input_to_string();
        assert_eq!(p2(&mut input), 96317);
    }
}
