use std::ops::Range;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    // let rows = input.lines().map(|line| {
    //     let (conditions, groups) = line.split_once(' ').unwrap();
    //     let groups: Vec<usize> = groups
    //         .split(',')
    //         .map(|g| g.parse().unwrap())
    //         .sorted()
    //         .rev()
    //         .collect();
    //     let ranges = group_ranges(conditions.as_bytes());
    //     (ranges, groups)
    // });

    let rows: Vec<(Vec<&[u8]>, Vec<usize>)> = vec![(vec![b"????", b"ABC"], vec![3, 2])];
    let rows = rows.into_iter();

    for (ranges, groups) in rows.skip(0).take(1) {
        count_arrangements(0, &groups, ranges);
    }

    0
}

fn count_arrangements(group_idx: usize, groups: &[usize], initial_ranges: Vec<&[u8]>) -> usize {
    // Base case if groups or ranges are empty
    if groups.is_empty() {
        let num_arrangements = !(groups.is_empty() ^ initial_ranges.is_empty()) as usize;
        _out(group_idx, num_arrangements);
        return num_arrangements;
    }

    _in(group_idx, groups[0], &initial_ranges);

    let group = groups[0];
    let next_group = *groups.get(1).unwrap_or(&usize::MAX);
    let ranges_ge_group = initial_ranges
        .iter()
        .positions(|r| r.len() >= group)
        .map(|i| (i, initial_ranges[i]));

    let mut num_arrangements = 0;
    for (range_idx, range) in ranges_ge_group {
        _try(group_idx, range);
        let ranges = clone_without(&initial_ranges, range_idx);

        if range.len() == group {
            let count = count_arrangements(group_idx + 1, &groups[1..], ranges);
            if group == next_group {
                num_arrangements = count;
            } else {
                num_arrangements += count;
            }
            continue;
        }

        // if range.len() == group + 1 {
        //     let count = count_arrangements(group_idx + 1, &groups[1..], ranges);
        //     if group == next_group {
        //         num_arrangements += count;
        //     } else {
        //         num_arrangements += 2 * count;
        //     }
        //     continue;
        // }

        for start in 0..(range.len() - group + 1) {
            let end = (start + group + 1).min(range.len());
            let start = start.saturating_sub(1);

            let subrange = &range[start..end];
            _subrange(group_idx, subrange);

            let mut sub_ranges = ranges.clone();

            let (left, right) = range.split_at_range_unchecked(&(start..end));
            println!(
                "{}│──left {:?} ,right: {:?}",
                "│  ".repeat(group_idx),
                rts(left),
                rts(right)
            );

            if !left.is_empty() {
                sub_ranges.push(left);
            }
            if !right.is_empty() {
                sub_ranges.push(right);
            }

            num_arrangements += count_arrangements(group_idx + 1, &groups[1..], sub_ranges);
        }
    }

    _out(group_idx, num_arrangements);
    num_arrangements
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

fn rts(range: &[u8]) -> &str {
    range.as_str_unchecked()
}

fn srts<'a>(ranges: &'a [&[u8]]) -> Vec<&'a str> {
    ranges.iter().map(|r| rts(r)).collect()
}

fn clone_without<'a>(vec: &'a Vec<&[u8]>, idx: usize) -> Vec<&'a [u8]> {
    let mut vec = vec.clone();
    vec.swap_remove(idx);
    vec
}

fn _in(idx: usize, group: usize, initial_ranges: &Vec<&[u8]>) {
    println!(
        "{}{}─in {:?}",
        "│  ".repeat(idx),
        group,
        srts(initial_ranges)
    );
}

fn _try(idx: usize, range: &[u8]) {
    println!("{}├try {:?}", "│  ".repeat(idx), rts(range));
}

fn _out(idx: usize, count: usize) {
    println!("{}└out {}", "│  ".repeat(idx), count);
}

fn _subrange(idx: usize, subrange: &[u8]) {
    println!("{}│─sub {:?}", "│  ".repeat(idx), rts(subrange));
}
