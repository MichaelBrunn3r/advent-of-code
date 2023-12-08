use aoc_2023_7::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution(0, task_0);
    }

    #[test]
    fn test_example_1() {
        aoc::assert_solution(1, task_1);
    }

    #[test]
    fn test_input() {
        let input = aoc::read_input_to_string();
        assert_eq!(task_0(&input), 252295678);
        assert_eq!(task_1(&input), 250577259);
    }
}
