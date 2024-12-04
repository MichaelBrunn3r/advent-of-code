use aoc_2023_8::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (instructions, network, nodes_ending_in_a) = parse(&input);

    println!("Part 1: {}", p1(instructions, network));
    println!(
        "Part 2: {}",
        p2(instructions, network, &nodes_ending_in_a)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = aoc::read_input_to_string();
        let (instructions, network, _) = parse(&input);

        assert_eq!(p1(instructions, network), 16697);
    }

    #[test]
    fn test_p2() {
        let input = aoc::read_input_to_string();
        let (instructions, network, nodes_ending_in_a) = parse(&input);

        assert_eq!(
            p2(instructions, network, &nodes_ending_in_a),
            10668805667831
        );
    }
}
