use iter::{SeriesValuesIterator, SeriesValuesIteratorReverse};
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
        for i in 1..end {
            series[i - 1] = series[i] - series[i - 1];
        }

        end -= 1;

        if series[0] == 0 && series[end] == 0 || end == 0 {
            break;
        }
    }

    series.iter().fold(0, |acc, f| f + acc)
}
