use aoc::prelude::*;
use aoc_2023_5::RangeToRangeMap;
use itertools::Itertools;
use std::ops::Range;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Task 0: {}", task_0(&input));
    println!("Task 1: {}", task_1(&input));
}

fn task_0(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let seeds: Vec<usize> = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse_splits(" ")
        .collect();

    // println!("{:?}", seeds);

    let map_sections = sections
        .map(|s| parse_map_section(s.lines()))
        .collect::<Vec<_>>();

    let mut min = usize::MAX;
    for seed in seeds.iter() {
        let mut mapped = *seed;
        for mappings in map_sections.iter() {
            let default = RangeToRangeMap::identity(mapped);
            let mapping = mappings
                .maps
                .iter()
                .take_while(|map| mapped >= map.from.start)
                .find(|map| map.from.contains(&mapped))
                .unwrap_or(&default);

            mapped = mapping.map_value(mapped);
        }
        if mapped < min {
            min = mapped;
        }
    }

    min
}

fn task_1(input: &str) -> usize {
    let mut sections = input.split("\n\n");

    let mut seed_ranges: Vec<Range<usize>> = sections
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

    let map_sections = sections
        .map(|s| parse_map_section(s.lines()))
        .collect::<Vec<_>>();

    for map_section in map_sections.iter() {
        seed_ranges = seed_ranges
            .into_iter()
            .map(|mut seed_range| {
                let maps: Vec<&RangeToRangeMap> = map_section
                    .maps
                    .iter()
                    .filter(|m| seed_range.overlaps(&m.from))
                    .collect();

                if maps.is_empty() {
                    return vec![seed_range];
                }

                let mut mapped = vec![];
                for map in maps.into_iter() {
                    let (left_overhang, mapped_range, right_overhang) = map.map(seed_range);
                    mapped.push(mapped_range);

                    match left_overhang {
                        // Maps are sorted -> We won't find a mapping for the left overhang in this section
                        Some(range) => mapped.push(range),
                        _ => {}
                    }

                    if right_overhang.is_none() {
                        break;
                    }

                    // Maps are sorted -> right overhang may be mapped by subsequent maps
                    seed_range = right_overhang.unwrap();
                }

                mapped
            })
            .flatten()
            .collect();
    }

    seed_ranges.iter().map(|r| r.start).min().unwrap()
}

fn parse_map_section<'a>(mut lines: impl Iterator<Item = &'a str>) -> MapSection<'a> {
    let (from, to) = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .0
        .split_once("-to-")
        .unwrap();

    let mut maps: Vec<RangeToRangeMap> = lines
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

    MapSection { from, to, maps }
}

struct MapSection<'a> {
    from: &'a str,
    to: &'a str,
    maps: Vec<RangeToRangeMap>,
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
