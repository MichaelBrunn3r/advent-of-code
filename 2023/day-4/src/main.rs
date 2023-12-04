use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    0
}

fn task_1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_0() {
        let input = std::fs::read_to_string("example_0").unwrap();
        let expected = std::fs::read_to_string("solution_0")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        assert_eq!(task_0(&input), expected);
    }

    // #[test]
    // fn test_task_1() {
    //     let input = std::fs::read_to_string("example_1").unwrap();
    //     let expected = std::fs::read_to_string("solution_1")
    //         .unwrap()
    //         .parse::<usize>()
    //         .unwrap();
    //     assert_eq!(task_1(&input), expected);
    // }
}
