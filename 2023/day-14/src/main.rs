use aoc::prelude::*;
use aoc_2023_14::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&mut aoc::read_input_to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        aoc::assert_solution(0, part_1);
    }

    #[test]
    fn test_1_cycle() {
        let mut input = aoc::read_example_to_string(0);
        let size = input.lines().next().unwrap().len();
        let platform = unsafe { input.as_bytes_mut() };

        spin(platform, size, size + 1);
        let expected = aoc::read_example_to_string(1);
        assert_eq!(platform.as_str_unchecked(), expected);
    }

    #[test]
    fn test_2_cycles() {
        let mut input = aoc::read_example_to_string(0);
        let size = input.lines().next().unwrap().len();
        let platform = unsafe { input.as_bytes_mut() };

        spin(platform, size, size + 1);
        spin(platform, size, size + 1);
        let expected = aoc::read_example_to_string(2);
        assert_eq!(platform.as_str_unchecked(), expected);

        println!("{}", platform.as_str_unchecked());
        assert_eq!(calc_load(platform, size), 69);
    }

    #[test]
    fn test_3_cycles() {
        let mut input = aoc::read_example_to_string(0);
        let size = input.lines().next().unwrap().len();
        let platform = unsafe { input.as_bytes_mut() };

        spin(platform, size, size + 1);
        spin(platform, size, size + 1);
        spin(platform, size, size + 1);
        let expected = aoc::read_example_to_string(3);
        assert_eq!(platform.as_str_unchecked(), expected);

        println!("{}", platform.as_str_unchecked());
        assert_eq!(calc_load(platform, size), 69);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(part_2(&mut aoc::read_example_to_string(0)), 64);
    }

    #[test]
    fn test_part_1() {
        let input = aoc::read_input_to_string();
        assert_eq!(part_1(&input), 107430);
    }

    #[test]
    fn test_part_2() {
        let mut input = aoc::read_input_to_string();
        assert_eq!(part_2(&mut input), 96317);
    }

    #[test]
    fn test_tilt_and_or_then_calc() {
        let input = aoc::read_input_to_string();
        let size = input.lines().next().unwrap().len();

        // Tilt then calc
        let mut platform = input.clone();
        let platform = unsafe { platform.as_bytes_mut() };
        tilt_north(platform, size, size + 1);
        let tilt_then_calc = calc_load(platform, size);

        // Tilt and calc at the same time
        let mut platform = input.clone();
        let platform = unsafe { platform.as_bytes_mut() };
        let tilt_and_calc = tilt_north_and_calc_load(platform, size);

        assert_eq!(tilt_then_calc, tilt_and_calc);
    }
}
