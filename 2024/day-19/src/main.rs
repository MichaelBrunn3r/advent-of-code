use aoc_2024_19::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (patterns, designs) = parse(&input);
    println!("Part 1&2: {:?}", p(&patterns, designs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        let input = aoc::read_input_to_string();
        let (patterns, designs) = parse(&input);
        assert_eq!(p(&patterns, designs), (220, 565600047715343));
    }
}