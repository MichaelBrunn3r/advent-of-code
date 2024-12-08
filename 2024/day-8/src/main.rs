use aoc_2024_8::*;

fn main() {
    let node_locations = parse(&aoc::read_input_to_string());
    println!("Part 1&2: {:?}", p(&node_locations));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        assert_eq!(p(&parse(&aoc::read_input_to_string())), (293, 934));
    }
}