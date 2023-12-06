use aoc::prelude::*;
use itertools::Itertools;
use std::ops::Range;

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

    let mut min = usize::MAX;
    for seed in seeds.iter() {
        let mut mapped = *seed;
        for mappings in maps.iter() {
            let default = RangeToRangeMap::identity(mapped);
            let mapping = mappings
                .iter()
                .take_while(|map| mapped >= map.from.start)
                .find(|map| map.from.contains(&mapped))
                .unwrap_or(&default);

            mapped = mapping.apply(mapped);
        }
        if mapped < min {
            min = mapped;
        }
    }

    min
}

fn task_1(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let seed_ranges: Vec<Range<usize>> = sections
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
            start..start + len
        })
        .collect();

    let maps = sections
        .map(|s| parse_map_section(s.lines()))
        .collect::<Vec<_>>();

    let mut min = usize::MAX;
    for seed_range in seed_ranges.iter() {
        println!("Seed range={:?}", seed_range);
        for seed in seed_range.clone().progress(seed_range.len().into()) {
            let mut mapped = seed;
            for mappings in maps.iter() {
                let default = RangeToRangeMap::identity(mapped);
                let mapping = mappings
                    .iter()
                    .take_while(|map| mapped >= map.from.start)
                    .find(|map| map.from.contains(&mapped))
                    .unwrap_or(&default);

                mapped = mapping.apply(mapped);
            }

            if mapped < min {
                min = mapped;
            }
        }
    }

    min
}

fn parse_map_section<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<RangeToRangeMap> {
    let mut maps: Vec<RangeToRangeMap> = lines
        .skip(1) // Skip header
        .map(|line| {
            let (to_start, from_start, len) = line
                .parse_splits::<usize>(" ")
                .next_tuple::<(usize, usize, usize)>()
                .unwrap();
            RangeToRangeMap {
                from: (from_start)..(from_start + len),
                to: (to_start)..(to_start + len),
            }
        })
        .collect();

    maps.sort_by_key(|map| map.from.start);

    maps
}

#[derive(Debug)]
struct RangeToRangeMap {
    from: Range<usize>,
    to: Range<usize>,
}

impl From<(Range<usize>, Range<usize>)> for RangeToRangeMap {
    fn from((from, to): (Range<usize>, Range<usize>)) -> Self {
        Self { from, to }
    }
}

impl RangeToRangeMap {
    fn identity(val: usize) -> Self {
        Self {
            from: val..val,
            to: val..val,
        }
    }

    fn apply(&self, val: usize) -> usize {
        self.to.start + (val - self.from.start)
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
