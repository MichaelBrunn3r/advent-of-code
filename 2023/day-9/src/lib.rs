use iter::{SeriesValuesIterator, SeriesValuesIteratorReverse};
use itertools::Itertools;
mod iter;

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_next_value(SeriesValuesIterator::new(line.as_bytes())))
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_next_value(SeriesValuesIteratorReverse::new(line.as_bytes())))
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

fn parse_i32(mut input: &[u8]) -> i32 {
    let mut val = 0;

    let sign = if input[0] == b'-' {
        input = &input[1..];
        -1
    } else {
        1
    };

    for c in input {
        val = val * 10 + (c - b'0') as i32;
    }

    val * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32(b"123"), 123);
        assert_eq!(parse_i32(b"-123"), -123);
    }
}
