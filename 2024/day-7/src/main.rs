use aoc_2024_7::*;

fn main() {
    let input = aoc::read_input_to_string();
    let lines = parse(&input);
    println!("Part 1: {}", p1(&lines));
    println!("Part 2: {}", p2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let lines = parse(&aoc::read_input_to_string());
        assert_eq!(p1(&lines), 663613490587);
    }

    #[test]
    fn test_p2() {
        let lines = parse(&aoc::read_input_to_string());
        assert_eq!(p2(&lines), 110365987435001);
    }
}