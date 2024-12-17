use aoc_2024_17::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_input_to_string()), "2,1,0,4,6,2,4,2,0");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_input_to_string()), 0);
    }
}