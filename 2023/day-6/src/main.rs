use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    let re_number = Regex::new(r"(\d+)").unwrap();
    let (times, distances_to_beat) = input
        .split('\n')
        .map(|line| {
            re_number
                .find_iter(line.split_once(':').unwrap().1)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect_tuple()
        .unwrap();

    times
        .iter()
        .zip(distances_to_beat.iter())
        .map(|(time, distance_to_beat)| {
            let (max_hold_time, min_hold_time) =
                quadratic_formula(1f32, -(*time as f32), *distance_to_beat as f32);

            (
                (max_hold_time - 1.0).ceil() as usize,
                (min_hold_time + 1.0).floor() as usize,
            )
        })
        .map(|(max_hold_time, min_hold_time)| max_hold_time - min_hold_time + 1)
        .reduce(|acc, x| acc * x)
        .unwrap()
}

fn task_1(input: &str) -> usize {
    0
}

fn quadratic_formula(a: f32, b: f32, c: f32) -> (f32, f32) {
    let root = (b.powf(2f32) - 4f32 * a * c).sqrt();
    let x1 = (-b + root) / 2f32 * a;
    let x2 = (-b - root) / 2f32 * a;

    (x1, x2)
}

fn calculate_distance(time: usize, hold_time: usize) -> usize {
    hold_time * (time - hold_time)
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
}
