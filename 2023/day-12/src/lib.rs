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

    for (ranges, groups) in rows.skip(1).take(1) {
        let arrangements = count_arrangements(0, &groups, ranges);
        println!("#Arrangements: {}", arrangements);
    }

    0
}

fn count_arrangements(group_idx: usize, groups: &[usize], ranges: Vec<&[u8]>) -> usize {
    let group = groups[group_idx];
    println!(
        "{}{} in={:?}",
        "    ".repeat(group_idx),
        group,
        ranges.as_strs_unchecked()
    );

    if ranges.len() == 0 {
        return 0;
    } else if group_idx == groups.len() - 1 {
        let range = ranges[0];
        if range.len() == group {
            return 1;
        }

        if range[0] != b'?' && range[range.len() - 1] != b'?' {
            return 0;
        }

        println!("{}  found", "    ".repeat(group_idx));
        return 1;
    }

    let mut arrangements = 0;
    for range_idx in ranges.iter().positions(|r| r.len() >= group) {
        let mut next_ranges = ranges.clone();
        let range = next_ranges.swap_remove(range_idx);
        println!(
            "{}  try: {:?}",
            "    ".repeat(group_idx),
            range.as_str_unchecked()
        );

        if range.len() == group {
            arrangements += count_arrangements(group_idx + 1, &groups, next_ranges);
        } else {
            let mut sub_arrangements = 0;
            for middle in 0..range.len() {
                let start = middle.saturating_sub(1);
                let end = (middle + group + 1).min(range.len());

                if (middle == start && range[start] != b'?')
                    || (middle == end && range[end - 1] != b'?')
                {
                    continue;
                }

                println!(
                    "{}   subrange {:?}",
                    "    ".repeat(group_idx),
                    range[start..end].as_str_unchecked()
                );

                let mut next_ranges_sub = next_ranges.clone();
                let (left, right) = range.split_at_range_unchecked(&(start..end));

                println!(
                    "{}   left: {:?} right: {:?}",
                    "    ".repeat(group_idx),
                    left.as_str_unchecked(),
                    right.as_str_unchecked()
                );

                if left.len() > 0 {
                    next_ranges_sub.push(left);
                }
                if right.len() > 0 {
                    next_ranges_sub.push(right);
                }

                let count = count_arrangements(group_idx + 1, &groups, next_ranges_sub);
                // println!("{}   count: {}", "    ".repeat(group_idx), count);
                if group == groups[group_idx + 1] {
                    sub_arrangements = sub_arrangements.max(count);
                } else {
                    sub_arrangements += count;
                }
            }
            println!(
                "{}  sub_arrangements: {}",
                "    ".repeat(group_idx),
                sub_arrangements
            );
            arrangements += sub_arrangements;
        }
    }

    arrangements
}

pub fn part_2(input: &str) -> usize {
    0
}

fn group_ranges(conditions: &[u8]) -> Vec<&[u8]> {
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
        possible_ranges.push(&conditions[start..end]);
        start = end + 1;
    }

    possible_ranges
}
