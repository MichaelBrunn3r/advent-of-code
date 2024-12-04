use aoc_2023_17::*;

fn main() {
    let input = aoc::read_input_to_string();
    // println!("Part 1: {}", p1(&mut aoc::read_input_to_string()));
    println!("Part 1: {}", p1(&mut aoc::read_example_to_string(0)));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution_mut(0, p1);
    }

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, p2);
    }
}
