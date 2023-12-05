use aoc;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    let mut maps = input.split("\n\n");

    let seeds: Vec<usize> = maps
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    // println!("{:?}", seeds);

    let mappings = maps
        .map(|s| s.lines())
        .map(|mut lines| {
            let (from, to) = lines
                .next()
                .unwrap()
                .split_once(' ')
                .unwrap()
                .0
                .split_once("-to-")
                .unwrap();
            (from, to, lines)
        })
        // .inspect(|(from, to, _)| println!("{} -> {}", from, to))
        .map(|(from, to, lines)| {
            let mappings: Vec<(usize, usize, usize)> = lines
                .map(|line| {
                    line.split(' ')
                        .map(|s| s.parse::<usize>().unwrap())
                        .next_tuple()
                        .unwrap()
                })
                .collect();
            (from, to, mappings)
        })
        // .inspect(|(_, _, mappings)| println!("{:?}", mappings))
        .collect::<Vec<_>>();

    let mut locations = seeds.clone();

    for locations in locations.iter_mut() {
        for (_, _, mappings) in mappings.iter() {
            let default = (*locations, *locations, 1);
            let mapping = mappings
                .iter()
                .find(|(_, source, len)| *locations >= *source && *locations <= *source + len)
                .unwrap_or(&default);

            *locations = mapping.0 + (*locations - mapping.1)
        }
    }

    seeds
        .iter()
        .enumerate()
        .map(|(i, _)| locations[i])
        .min()
        .unwrap()
}

fn task_1(input: &str) -> usize {
    0
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
