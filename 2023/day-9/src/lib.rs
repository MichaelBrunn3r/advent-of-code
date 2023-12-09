// #![feature(portable_simd)]

use iter::{SeriesValuesIterator, SeriesValuesIteratorReverse};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
// use std::simd::i;

mod iter;

pub fn part_1(input: &str) -> i32 {
    let mut buffer = Vec::<i32>::with_capacity(21);
    let mut sum = 0;

    for line in input.lines() {
        buffer.extend(SeriesValuesIterator::new(line.as_bytes()));
        sum += predict_next_value(&mut buffer);
        buffer.clear();
    }

    sum
}

pub fn part_2(input: &str) -> i32 {
    let mut buffer = Vec::<i32>::with_capacity(21);
    let mut sum = 0;

    for line in input.lines() {
        buffer.extend(SeriesValuesIteratorReverse::new(line.as_bytes()));
        sum += predict_next_value(&mut buffer);
        buffer.clear();
    }

    sum
}

fn predict_next_value(series: &mut Vec<i32>) -> i32 {
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
