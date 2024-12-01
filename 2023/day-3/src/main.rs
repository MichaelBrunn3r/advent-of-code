use aoc_2023_3::{part_1, part_2, prepare_input};

fn main() {
    let input = aoc::read_input_to_string();
    let lines = prepare_input(&input);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&lines));
}

#[cfg(test)]
mod tests {
    use aoc_2023_3::prepare_input;

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&aoc::read_input_to_string()), 527369);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&prepare_input(&aoc::read_input_to_string())),
            73074886
        );
    }
}
