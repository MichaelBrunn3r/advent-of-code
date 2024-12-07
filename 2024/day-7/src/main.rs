use aoc_2024_7::*;

fn main() {
    println!("Part 1&2: {:?}", p(&parse(&aoc::read_input_to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p(&parse(&aoc::read_input_to_string())), (663613490587, 110365987435001));
    }
}