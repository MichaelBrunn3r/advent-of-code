use aoc_2023_20::{parse::parse, *};

fn main() {
    let input = aoc::read_input_to_string();
    let (broadcaster_outputs, cycle_conjunctions, modules) = parse(&input);

    println!(
        "Part 1: {}",
        p1(&broadcaster_outputs, modules, &cycle_conjunctions)
    );
    println!(
        "Part 2: {}",
        p2(&broadcaster_outputs, modules, &cycle_conjunctions)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let (broadcaster_outputs, cycle_conjunctions, modules) =
            parse(&aoc::read_input_to_string());
        assert_eq!(
            p1(&broadcaster_outputs, modules, &cycle_conjunctions),
            666795063
        );
    }

    #[test]
    fn test_p2() {
        let (broadcaster_outputs, cycle_conjunctions, modules) =
            parse(&aoc::read_input_to_string());
        assert_eq!(
            p2(&broadcaster_outputs, modules, &cycle_conjunctions),
            253302889093151
        );
    }
}
