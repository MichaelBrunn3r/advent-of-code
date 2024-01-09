use aoc::U8PtrExt;

const NUM_HISTORIES: usize = 200;
const VALUES_PER_HISTORY: usize = 21;

static mut DATA: [[i32; VALUES_PER_HISTORY]; NUM_HISTORIES] = unsafe { std::mem::zeroed() };
type Data = [[i32; VALUES_PER_HISTORY]; NUM_HISTORIES];

pub fn parse(input: &str) -> &'static Data {
    let mut data = input.as_ptr();
    unsafe {
        for history in &mut DATA {
            for val in history {
                *val = data.parse_int::<i32, 1>();
                data = data.add(1);
            }
        }

        &mut DATA
    }
}

pub fn part_1(data: &Data) -> i32 {
    let mut buffer = [0; VALUES_PER_HISTORY];
    let mut sum = 0;

    for values in data {
        buffer.copy_from_slice(&values[..VALUES_PER_HISTORY]);
        sum += predict_next_value(&mut buffer);
    }

    sum
}

pub fn part_2(data: &Data) -> i32 {
    let mut buffer = [0; VALUES_PER_HISTORY];
    let mut sum = 0;

    for values in data {
        for (i, val) in values.iter().rev().enumerate() {
            buffer[i] = *val;
        }
        sum += predict_next_value(&mut buffer);
    }

    sum
}

fn predict_next_value(series: &mut [i32]) -> i32 {
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
