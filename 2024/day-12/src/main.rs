use aoc_2024_12::*;

fn main() {
    let mut input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&mut input));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&mut aoc::read_input_to_string()), 1400386);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_input_to_string()), 0);
    }
}