use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let mut predictions = vec![];

    for line in input.lines() {
        let (mut last, mut all_zero, mut diffs) =
            calc_differences(line.split(' ').map(|num| num.parse::<i32>().unwrap()));

        let mut last_values = vec![last];

        while !all_zero {
            (last, all_zero, diffs) = calc_differences(diffs.into_iter());
            last_values.push(last);
        }

        predictions.push(last_values.iter().fold(0, |acc, f| acc + f));
    }

    predictions.iter().sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

fn calc_differences(series: impl Iterator<Item = i32>) -> (i32, bool, Vec<i32>) {
    let mut last = 0;
    let mut all_zero = true;

    let diff1 = series
        .inspect(|val| last = *val)
        .tuple_windows()
        .map(|(a, b)| b - a)
        .inspect(|diff| all_zero &= *diff == 0)
        .collect();

    (last, all_zero, diff1)
}
