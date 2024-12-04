use aoc_2023_9::*;

fn main() {
    let input = aoc::read_input_to_string();
    let data = parse(&input);

    println!("Part 1: {}", p1(data));
    // println!("Part 2: {}", p2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(parse(&aoc::read_input_to_string())), 1637452029);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(parse(&aoc::read_input_to_string())), 908);
    }
}
