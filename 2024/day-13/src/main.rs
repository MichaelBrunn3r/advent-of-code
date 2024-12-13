use aoc_2024_13::*;

fn main() {
    println!("Part 1&2: {:?}", p(&aoc::read_input_to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        assert_eq!(p(&aoc::read_input_to_string()), (29187, 99968222587852));
    }
}