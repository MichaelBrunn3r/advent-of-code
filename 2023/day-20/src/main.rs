#![allow(unused_imports, unused_variables)]

use aoc_2023_20::*;

fn main() {
    let input = aoc::read_input_to_string();
    let (broadcaster_outputs, cycle_conjunctions, modules) = parse(&input);

    println!(
        "Part 1: {}",
        part_1(&broadcaster_outputs, modules, &cycle_conjunctions)
    );
    println!(
        "Part 2: {}",
        part_2(&broadcaster_outputs, modules, &cycle_conjunctions)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (broadcaster_outputs, cycle_conjunctions, modules) =
            parse(&aoc::read_input_to_string());
        assert_eq!(
            part_1(&broadcaster_outputs, modules, &cycle_conjunctions),
            666795063
        );
    }

    #[test]
    fn test_part_2() {
        let (broadcaster_outputs, cycle_conjunctions, modules) =
            parse(&aoc::read_input_to_string());
        assert_eq!(
            part_2(&broadcaster_outputs, modules, &cycle_conjunctions),
            253302889093151
        );
    }
}
