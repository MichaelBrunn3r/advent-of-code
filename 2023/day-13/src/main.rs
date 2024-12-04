use aoc_2023_13::*;

fn main() {
    let input = aoc::read_input_to_string();
    let patterns = parse(&input);

    println!("Part 1: {}", p1(&patterns));
    println!("Part 2: {}", p2(&patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
        assert_eq!(p1(&parse(&input)), 33728);
    }

    #[test]
    fn test_p2() {
        let input = aoc::read_input_to_string();
        assert_eq!(p2(&parse(&input)), 28235);
    }
}
