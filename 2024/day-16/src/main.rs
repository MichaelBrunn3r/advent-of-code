use aoc_2024_16::*;

fn main() {
    let mut input = aoc::read_input_to_string();
    println!("Part 1&2: {:?}", p(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        assert_eq!(p(&mut aoc::read_input_to_string()), (74392, 426));
    }
}