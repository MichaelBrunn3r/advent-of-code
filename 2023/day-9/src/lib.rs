// #![feature(stdsimd)]
use aoc::U8PtrExt;
// use std::arch::x86_64::{_mm256_loadu_epi32, _mm256_storeu_epi32, _mm256_sub_epi32};

const NUM_HISTORIES: usize = 200;
const VALUES_PER_HISTORY: usize = 21;
const NUM_VALUES: usize = NUM_HISTORIES * VALUES_PER_HISTORY;

type Data = [i32; NUM_VALUES + 201];
static mut DATA: Data = unsafe { std::mem::zeroed() };

pub fn p1(data: &Data) -> i32 {
    let mut buffer: Data = unsafe { std::mem::zeroed() };
    buffer.copy_from_slice(data);
    predict_and_sum(&mut buffer)
}

pub fn p2(data: &Data) -> i32 {
    let mut buffer: Data = unsafe { std::mem::zeroed() };
    for (i, val) in data.iter().rev().skip(2).enumerate() {
        buffer[i] = *val;
    }
    predict_and_sum(&mut buffer)
}

fn predict_and_sum(series: &mut Data) -> i32 {
    for _ in 0..19 {
        for i in 1..series.len() {
            series[i - 1] = series[i].wrapping_sub(series[i - 1]);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    for _ in 0..NUM_HISTORIES {
        sum += -series[i + 2];
        i += VALUES_PER_HISTORY + 1;
    }

    sum
}

// fn predict_next_value_simd(series: &mut Data) -> i32 {
//     unsafe {
//         for pass in 0..19 {
//             let mut i = 0;
//             for _ in 0..550 {
//                 _mm256_storeu_epi32(
//                     series.as_mut_ptr().add(i),
//                     _mm256_sub_epi32(
//                         _mm256_loadu_epi32(series.as_ptr().add(i + 1)),
//                         _mm256_loadu_epi32(series.as_ptr().add(i)),
//                     ),
//                 );
//                 i += 8;
//             }
//         }
//     }

//     let mut sum = 0;
//     let mut i = 0;
//     for _ in 0..NUM_HISTORIES {
//         sum += -series[i + 2];
//         i += VALUES_PER_HISTORY + 1;
//     }

//     sum
// }

pub fn parse(input: &str) -> &'static Data {
    let mut data = input.as_ptr();
    unsafe {
        let mut i = 0;
        for _ in 0..NUM_HISTORIES {
            // Unrolled loop 21 times. This seems to be the fastest way to parse the input.
            DATA[i] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 1] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 2] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 3] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 4] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 5] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 6] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 7] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 8] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 9] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 10] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 11] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 12] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 13] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 14] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 15] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 16] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 17] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 18] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 19] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);
            DATA[i + 20] = if data.read() == b'-' {
                data = data.add(1);
                -data.parse_uint::<i32, 1>()
            } else {
                data.parse_uint::<i32, 1>()
            };
            data = data.add(1);

            i += VALUES_PER_HISTORY + 1;
        }

        &mut DATA
    }
}
