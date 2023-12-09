use aoc_2023_8::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        unsafe {
            NETWORK.fill((0, 0));
        }
        aoc::assert_solution(0, part_1);
    }

    #[test]
    fn test_example_1() {
        unsafe {
            NETWORK.fill((0, 0));
        }
        aoc::assert_solution(1, part_1);
    }

    #[test]
    fn test_example_2() {
        unsafe {
            NETWORK.fill((0, 0));
        }
        aoc::assert_solution(2, part_2);
    }
}
