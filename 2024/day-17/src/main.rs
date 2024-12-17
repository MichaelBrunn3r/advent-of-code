use aoc_2024_17::*;

fn main() {
    let (a, prog) = parse(&aoc::read_input_to_string());
    println!("Part 1: {}", p1(a, &prog));
    println!("Part 2: {}", p2(a, &prog));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let (a, prog) = parse(&aoc::read_input_to_string());
        assert_eq!(p1(a, &prog), "2,1,0,4,6,2,4,2,0");
    }

    #[test]
    fn test_p2() {
        let (a, prog) = parse(&aoc::read_input_to_string());
        assert_eq!(p2(a, &prog), 0);
    }
}