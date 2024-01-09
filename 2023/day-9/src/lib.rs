// #![feature(stdsimd)]
// use std::arch::x86_64::{_mm256_loadu_epi32, _mm256_storeu_epi32, _mm256_sub_epi32};
use aoc::U8PtrExt;

const NUM_HISTORIES: usize = 200;
const VALUES_PER_HISTORY: usize = 21;

type Data = [[i32; VALUES_PER_HISTORY]; NUM_HISTORIES];
static mut DATA: Data = unsafe { std::mem::zeroed() };

pub fn parse(input: &str) -> &'static Data {
    let mut data = input.as_ptr();
    unsafe {
        for history in &mut DATA {
            for val in history {
                if data.read() == b'-' {
                    data = data.add(1);
                    *val = -data.parse_int::<i32, 1>();
                } else {
                    *val = data.parse_uint::<i32, 1>();
                }
                data = data.add(1);
            }
        }

        &mut DATA
    }
}

pub fn part_1(data: &Data) -> i32 {
    let mut buffer = [0; VALUES_PER_HISTORY + 3];
    let mut sum = 0;

    for values in data {
        buffer[..VALUES_PER_HISTORY].copy_from_slice(&values[..VALUES_PER_HISTORY]);
        sum += predict_next_value(&mut buffer);
        // sum += predict_next_value_simd(values);
    }

    sum
}

pub fn part_2(data: &Data) -> i32 {
    let mut buffer = [0; VALUES_PER_HISTORY + 3];
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
    let mut end = VALUES_PER_HISTORY;

    loop {
        for i in 1..end + 1 {
            series[i - 1] = series[i] - series[i - 1];
        }

        end -= 1;

        if series[0] == 0 && series[end - 1] == 0 {
            break;
        }
    }

    -series[end]
}

// Slower than loop. Maybe because of unaligned loads?
// fn predict_next_value_simd(series: &[i32]) -> i32 {
//     let mut buffer = [0; VALUES_PER_HISTORY + 3];
//     buffer[..VALUES_PER_HISTORY].copy_from_slice(series);

//     unsafe {
//         let mut end = VALUES_PER_HISTORY - 1;

//         loop {
//             let a = _mm256_sub_epi32(
//                 _mm256_loadu_epi32(buffer.as_ptr().add(1)),
//                 _mm256_loadu_epi32(buffer.as_ptr()),
//             );
//             let b = _mm256_sub_epi32(
//                 _mm256_loadu_epi32(buffer.as_ptr().add(8 + 1)),
//                 _mm256_loadu_epi32(buffer.as_ptr().add(8)),
//             );
//             let c = _mm256_sub_epi32(
//                 _mm256_loadu_epi32(buffer.as_ptr().add(16 + 1)),
//                 _mm256_loadu_epi32(buffer.as_ptr().add(16)),
//             );

//             _mm256_storeu_epi32(buffer.as_mut_ptr(), a);
//             _mm256_storeu_epi32(buffer.as_mut_ptr().add(8), b);
//             _mm256_storeu_epi32(buffer.as_mut_ptr().add(16), c);

//             end -= 1;
//             if buffer[0] == 0 && buffer[end] == 0 || end == 0 {
//                 return -buffer[end + 1];
//             }
//         }
//     }
// }
