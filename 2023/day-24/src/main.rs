use aoc_2023_24::*;

fn main() {
    let input = aoc::read_input_to_string();
    let hailstones = parse(&input);
    println!("Part 1: {}", p1(hailstones));
    println!("Part 2: {}", p2(hailstones));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(parse(&aoc::read_input_to_string())), 20434);
    }

    // #[test]
    // fn test_p2() {
    //     assert_eq!(p2(&aoc::read_input_to_string()), 253302889093151);
    // }
}
