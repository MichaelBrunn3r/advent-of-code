use aoc_2024_13::*;

fn main() {
    let machines = parse(&aoc::read_input_to_string());
    println!("Part 1: {}", p1(&machines));
    println!("Part 2: {}", p2(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(&aoc::read_input_to_string())), 29187);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(&aoc::read_input_to_string())), 99968222587852);
    }
}