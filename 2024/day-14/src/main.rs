use aoc_2024_14::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1&2: {:?}", p(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        assert_eq!(p(&aoc::read_input_to_string()), (216027840, 6876));
    }
}