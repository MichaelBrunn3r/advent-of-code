#![allow(unused_imports, unused_variables)]

use aoc_2023_19::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (rules, workflows, workflow_in_id, parts, name_to_id) = parse(&input);

    println!(
        "Part 1: {}",
        part_1(&rules, &workflows, workflow_in_id, &parts)
    );
    println!("Part 2: {}", part_2(&rules, &workflows, &name_to_id));
    // println!("Part 1: {}", part_2(&aoc::read_example_to_string(0)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (rules, workflows, workflow_in_id, parts, _) = parse(&aoc::read_input_to_string());
        assert_eq!(part_1(&rules, &workflows, workflow_in_id, &parts), 418498)
    }

    #[test]
    fn test_part_2() {
        let input = aoc::read_input_to_string();
        let (rules, workflows, workflow_in_id, parts, name_to_id) = parse(&input);
        assert_eq!(part_2(&rules, &workflows, &name_to_id), 123331556462603)
    }
}
