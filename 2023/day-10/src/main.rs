use aoc_2023_10::*;

fn main() {
    println!("Part 1: {}", p1(&parse(aoc::read_input_to_string())));
    println!(
        "Part 2: {}",
        p2(&mut parse(aoc::read_input_to_string()))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(aoc::read_input_to_string())), 7145);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&mut parse(aoc::read_input_to_string())), 445);
    }
}
