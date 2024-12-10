use aoc_2024_10::*;

fn main() {
    println!("Part 1&2: {:?}", p(&aoc::read_input_to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        assert_eq!(p(&aoc::read_input_to_string()), (489, 1086));
    }
}
