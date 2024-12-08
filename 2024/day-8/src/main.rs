use aoc_2024_8::*;

fn main() {
    let node_locations = parse(&aoc::read_input_to_string());
    println!("Part 1: {}", p1(&node_locations));
    println!("Part 2: {}", p2(&node_locations));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(&aoc::read_input_to_string())), 293);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(&aoc::read_input_to_string())), 934);
    }
}