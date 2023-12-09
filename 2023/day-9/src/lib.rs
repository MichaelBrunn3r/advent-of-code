use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let mut predictions = vec![];

    for line in input.lines() {
        let (_, last_values) = collect_first_and_last_values(line);
        predictions.push(last_values.iter().fold(0, |acc, f| f + acc));
    }

    predictions.iter().sum()
}

pub fn part_2(input: &str) -> i32 {
    let mut predictions = vec![];

    for line in input.lines() {
        let (first_values, _) = collect_first_and_last_values(line);
        let prediction = first_values.iter().rev().fold(0, |acc, f| f - acc);
        predictions.push(prediction);
    }

    predictions.iter().sum()
}

fn calc_differences(series: Vec<i32>) -> (i32, i32, bool, Vec<i32>) {
    let last = series[series.len() - 1];
    let first = series[0];
    let mut all_zero = true;

    let mut diffs = vec![];

    for pair in series.windows(2) {
        let diff = pair[1] - pair[0];
        diffs.push(diff);
        all_zero &= diff == 0;
    }

    (first, last, all_zero, diffs)
}

fn collect_first_and_last_values(line: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut first, mut last, mut all_zero, mut diffs) = calc_differences(
        line.split(' ')
            .map(|num| num.parse::<i32>().unwrap())
            .collect_vec(),
    );

    let mut first_values = vec![first];
    let mut last_values = vec![last];

    while !all_zero {
        (first, last, all_zero, diffs) = calc_differences(diffs);
        last_values.push(last);
        first_values.push(first);
    }

    (first_values, last_values)
}
