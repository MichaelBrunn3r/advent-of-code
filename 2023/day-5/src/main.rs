use aoc::prelude::*;
use itertools::Itertools;
use std::ops::RangeInclusive;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    let mut section = input.split("\n\n");

    let seeds: Vec<usize> = section
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse_splits(" ")
        .collect();

    // println!("{:?}", seeds);

    let maps = section
        .map(|s| parse_map_section(s.lines()))
        .collect::<Vec<_>>();

    let mut locations = seeds.clone();

    for location in locations.iter_mut() {
        for mappings in maps.iter() {
            let default = RangeToRangeMap::identity(*location);
            let mapping = mappings
                .iter()
                .find(|map| map.from.contains(location))
                .unwrap_or(&default);

            *location = mapping.map(*location);
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
    let mut sections = input.split("\n\n");

    let seed_ranges: Vec<RangeInclusive<usize>> = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse_splits(" ")
        .chunks(2)
        .into_iter()
        .map(|mut c| {
            let (start, len) = c.next_tuple().unwrap();
            start..=start + len
        })
        .collect();

    let maps = sections
        .map(|s| parse_map_section(s.lines()))
        .collect::<Vec<_>>();

    let mut min = usize::MAX;
    for seed_range in seed_ranges.iter() {
        println!("Seed range={:?}", seed_range);
        for seed in seed_range.clone().progress(seed_range.len().into()) {
            // println!("Seed={}", seed);
            let mut current = seed;
            for mappings in maps.iter() {
                let default = RangeToRangeMap::identity(current);
                let mapping = mappings
                    .iter()
                    .find(|map| map.from.contains(&current))
                    .unwrap_or(&default);

                current = mapping.map(current);
                // println!("{:?}->{}", mapping, current);
            }

            if current < min {
                min = current;
            }
        }
    }

    min
}

fn parse_map_section<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<RangeToRangeMap> {
    lines
        .skip(1)
        .map(|line| {
            let (to_start, from_start, len) = line
                .parse_splits::<usize>(" ")
                .next_tuple::<(usize, usize, usize)>()
                .unwrap();
            RangeToRangeMap {
                from: (from_start)..=(from_start + len),
                to: (to_start)..=(to_start + len),
            }
        })
        .collect()
}

#[derive(Debug)]
struct RangeToRangeMap {
    from: RangeInclusive<usize>,
    to: RangeInclusive<usize>,
}

impl From<(RangeInclusive<usize>, RangeInclusive<usize>)> for RangeToRangeMap {
    fn from((from, to): (RangeInclusive<usize>, RangeInclusive<usize>)) -> Self {
        Self { from, to }
    }
}

impl RangeToRangeMap {
    fn identity(val: usize) -> Self {
        Self {
            from: val..=val,
            to: val..=val,
        }
    }

    fn map(&self, val: usize) -> usize {
        self.to.start() + (val - self.from.start())
    }
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
