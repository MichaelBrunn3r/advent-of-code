use std::ops::Range;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let rows = input.lines().map(|line| {
        let (conditions, groups) = line.split_once(' ').unwrap();
        let groups: Vec<usize> = groups
            .split(',')
            .map(|g| g.parse().unwrap())
            .sorted()
            .rev()
            .collect();
        let ranges = group_ranges(conditions.as_bytes());
        (ranges, groups)
    });

    for (ranges, groups) in rows {
        let arrangements = count_arrangements(0, &groups, ranges);
        println!("#Arrangements: {}", arrangements);
        break;
    }

    0
}

fn count_arrangements(idx: usize, groups: &[usize], ranges: Vec<Range<usize>>) -> usize {
    let group = groups[idx];
    println!("{}{} in={:?}", "    ".repeat(idx), group, ranges);

    if ranges.len() == 0 {
        return 0;
    } else if idx == groups.len() - 1 {
        if ranges[0].len() >= group {
            println!("{}  found", "    ".repeat(idx));
            return 1;
        } else {
            return 0;
        }
    }

    let mut arrangements = 0;
    let matches = ranges.iter().positions(|r| r.len() >= group).collect_vec();
    println!("{}  matches={:?}", "    ".repeat(idx), ranges);

    for match_idx in matches {
        let mut next_ranges = ranges.clone();
        let matching_range = next_ranges.swap_remove(match_idx);
        println!("{}  try: {:?}", "    ".repeat(idx), matching_range);

        if matching_range.len() == group {
            arrangements += count_arrangements(idx + 1, &groups, next_ranges);
        } else {
            for start in (matching_range.start)..(matching_range.end - group + 1) {
                let mut next_ranges_sub = next_ranges.clone();

                let subrange =
                    ((start.saturating_sub(1)).max(matching_range.start))..(start + group + 1);
                println!("{}   subrange {:?}", "    ".repeat(idx), subrange);
                let (left, right) = matching_range.without_unchecked(&subrange);
                if left.len() > 0 {
                    next_ranges_sub.push(left);
                }
                if right.len() > 0 {
                    next_ranges_sub.push(right);
                }

                arrangements += count_arrangements(idx + 1, &groups, next_ranges_sub);
            }
        }
    }

    arrangements
}

pub fn part_2(input: &str) -> usize {
    0
}

fn group_ranges(conditions: &[u8]) -> Vec<Range<usize>> {
    let mut possible_ranges = vec![];

    let mut start = 0;
    while start < conditions.len() {
        if conditions[start] == b'.' {
            start += 1;
            continue;
        }

        let mut end = start + 1;
        while end < conditions.len() && conditions[end] != b'.' {
            end += 1;
        }
        possible_ranges.push(start..end);
        start = end + 1;
    }

    possible_ranges
}
