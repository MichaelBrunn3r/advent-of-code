use aoc_2024_16::*;

fn main() {
    let mut input = aoc::read_input_to_string();
    println!("Part 1: {:?}", p1(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&mut aoc::read_input_to_string()), 74392);
    }
}