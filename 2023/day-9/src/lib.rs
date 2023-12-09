use itertools::Itertools;

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_next_value(line.split(' ').map(|num| num.parse::<i32>().unwrap())))
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            predict_next_value(line.split(' ').rev().map(|num| num.parse::<i32>().unwrap()))
        })
        .sum()
}

fn predict_next_value(series: impl Iterator<Item = i32>) -> i32 {
    let first_values = collect_first_and_last_values(series);
    first_values.iter().fold(0, |acc, f| f + acc)
}

fn calc_differences(series: impl Iterator<Item = i32>) -> (i32, bool, Vec<i32>) {
    let mut all_zero = true;
    let mut last = 0;

    let diffs = series
        .inspect(|x| last = *x)
        .tuple_windows()
        .map(|(a, b)| {
            let diff = b - a;
            all_zero &= diff == 0;
            diff
        })
        .collect_vec();

    (last, all_zero, diffs)
}

fn collect_first_and_last_values(series: impl Iterator<Item = i32>) -> Vec<i32> {
    let (mut last, mut all_zero, mut diffs) = calc_differences(series);

    let mut last_values = vec![last];

    while !all_zero {
        (last, all_zero, diffs) = calc_differences(diffs.into_iter());
        last_values.push(last);
    }

    last_values
}
