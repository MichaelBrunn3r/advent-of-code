use aoc_2023_3::{p1, p2, prepare_input};

fn main() {
    let input = aoc::read_input_to_string();
    let lines = prepare_input(&input);

    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&lines));
}

#[cfg(test)]
mod tests {
    use aoc_2023_3::prepare_input;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_input_to_string()), 527369);
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            p2(&prepare_input(&aoc::read_input_to_string())),
            73074886
        );
    }
}
