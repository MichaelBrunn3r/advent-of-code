use aoc_2024_15::*;

fn main() {
    let mut input = aoc::read_input_to_string();
    println!("Part 2: {}", p2(&input));
    println!("Part 1: {}", p1(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&mut aoc::read_input_to_string()), 1471826);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_input_to_string()), 0);
    }
}