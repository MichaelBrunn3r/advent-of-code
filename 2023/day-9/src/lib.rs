use itertools::Itertools;
pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_next_value(line.split(' ').map(|num| parse_i32(num.as_bytes()))))
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_next_value(line.split(' ').rev().map(|num| parse_i32(num.as_bytes()))))
        .sum()
}

fn predict_next_value(series: impl Iterator<Item = i32>) -> i32 {
    let mut series = series.collect_vec();
    let mut end = series.len();
    loop {
        let mut all_zero = false;
        for i in 1..end {
            let diff = series[i] - series[i - 1];
            all_zero &= diff == 0;
            series[i - 1] = diff;
        }

        end -= 1;

        if all_zero || end == 0 {
            break;
        }
    }

    series.iter().fold(0, |acc, f| f + acc)
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
