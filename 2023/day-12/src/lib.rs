use std::collections::HashMap;

use aoc::U8SliceExt;
use fxhash::{FxBuildHasher, FxHashMap};

const NUM_LINES: usize = 1000;
const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKOWN: u8 = b'?';

pub fn part_1(input: &str) -> usize {
    let mut runs = Vec::new();
    let mut memo = FxHashMap::with_capacity_and_hasher(9257, FxBuildHasher::default());

    let mut data = input.as_ptr();
    unsafe {
        let mut sum = 0;
        for _ in 0..NUM_LINES {
            let mut offset = 0;
            while data.add(offset).read() != b' ' {
                offset += 1;
            }
            let springs = std::slice::from_raw_parts(data, offset);

            data = data.add(offset + 1);
            let start = runs.len();
            let mut runs_sum = 0;
            while data.read().is_ascii_digit() {
                let mut num = data.read() - b'0';
                data = data.add(1);
                if data.read().is_ascii_digit() {
                    num = num * 10 + (data.read() - b'0');
                    data = data.add(1);
                }
                runs.push(num);
                data = data.add(1);
                runs_sum += num as usize;
            }
            let damaged_runs = &runs[start..];

            sum += count_arrangements(springs, damaged_runs, runs_sum, &mut memo);
        }
        sum
    }
}

// 6575118122726 too low
// 10776233067311 too high
pub fn part_2(input: &str) -> usize {
    let mut runs = Vec::new();
    let mut memo = FxHashMap::with_capacity_and_hasher(9257, FxBuildHasher::default());

    let mut data = input.as_ptr();
    unsafe {
        let mut sum = 0;
        for _ in 0..6 {
            let mut offset = 0;
            while data.add(offset).read() != b' ' {
                offset += 1;
            }
            let springs = std::slice::from_raw_parts(data, offset);
            let springs = format!(
                "{}?{}?{}?{}?{}",
                springs.as_str_unchecked(),
                springs.as_str_unchecked(),
                springs.as_str_unchecked(),
                springs.as_str_unchecked(),
                springs.as_str_unchecked()
            );
            println!("{}", springs);

            data = data.add(offset + 1);
            let start = runs.len();
            let mut runs_sum = 0;
            let mut current_runs = Vec::new();
            while data.read().is_ascii_digit() {
                let mut num = data.read() - b'0';
                data = data.add(1);
                if data.read().is_ascii_digit() {
                    num = num * 10 + (data.read() - b'0');
                    data = data.add(1);
                }
                current_runs.push(num);
                data = data.add(1);
                runs_sum += num as usize * 5;
            }
            for _ in 0..5 {
                runs.extend_from_slice(&current_runs);
            }
            let damaged_runs = &runs[start..];
            data = data.add(1);
            println!("{:?} {}", damaged_runs, runs_sum);

            let cnt = count_arrangements_2(springs.as_bytes(), damaged_runs, runs_sum, &mut memo);
            println!("{}", cnt);
            sum += cnt;
        }
        sum
    }
}

fn count_arrangements(
    springs: &[u8],
    damaged_runs: &[u8],
    damaged_runs_sum: usize,
    memo: &mut FxHashMap<(*const u8, u8), usize>,
) -> usize {
    if let Some(&result) = memo.get(&(springs.as_ptr(), damaged_runs.len() as u8)) {
        return result;
    }

    // No groups left
    if damaged_runs.is_empty() {
        if springs.iter().any(|c| *c == DAMAGED) {
            // There are some damaged springs left -> this arrangement is invalid
            return 0;
        } else {
            // All springs are operational or unknown -> this arrangement is valid
            return 1;
        }
    }

    if springs.len() < damaged_runs_sum + damaged_runs.len() - 1 {
        return 0;
    }

    if springs[0] == OPERATIONAL {
        return count_arrangements(&springs[1..], damaged_runs, damaged_runs_sum, memo);
    }

    let mut num_arrangements = 0;
    let run = damaged_runs[0] as usize;
    let all_not_operational = springs[..run].iter().all(|c| *c != OPERATIONAL);
    let end = (run + 1).min(springs.len());

    if all_not_operational
        && ((springs.len() > run && springs[run] != DAMAGED) || springs.len() <= run)
    {
        num_arrangements = count_arrangements(
            &springs[end..],
            &damaged_runs[1..],
            damaged_runs_sum - run,
            memo,
        );
    }

    if springs[0] == UNKOWN {
        num_arrangements += count_arrangements(&springs[1..], damaged_runs, damaged_runs_sum, memo);
    }

    memo.insert(
        (springs.as_ptr(), damaged_runs.len() as u8),
        num_arrangements,
    );
    num_arrangements
}

fn count_arrangements_2(
    springs: &[u8],
    damaged_runs: &[u8],
    damaged_runs_sum: usize,
    memo: &mut FxHashMap<(Vec<u8>, Vec<u8>), usize>,
) -> usize {
    // if let Some(&result) = memo.get(&(springs.to_vec(), damaged_runs.to_vec())) {
    //     return result;
    // }

    // No groups left
    if damaged_runs.is_empty() {
        if springs.iter().any(|c| *c == DAMAGED) {
            // There are some damaged springs left -> this arrangement is invalid
            return 0;
        } else {
            // All springs are operational or unknown -> this arrangement is valid
            return 1;
        }
    }

    if springs.len() < damaged_runs_sum + damaged_runs.len() - 1 {
        return 0;
    }

    if springs.is_empty() {
        return 0;
    }

    if springs[0] == OPERATIONAL {
        return count_arrangements_2(&springs[1..], damaged_runs, damaged_runs_sum, memo);
    }

    let mut num_arrangements = 0;
    let run = damaged_runs[0] as usize;
    let all_not_operational = springs[..run].iter().all(|c| *c != OPERATIONAL);
    let end = (run + 1).min(springs.len());

    if all_not_operational
        && ((springs.len() > run && springs[run] != DAMAGED) || springs.len() <= run)
    {
        num_arrangements = count_arrangements_2(
            &springs[end..],
            &damaged_runs[1..],
            damaged_runs_sum - run,
            memo,
        );
    }

    if springs[0] == UNKOWN {
        num_arrangements +=
            count_arrangements_2(&springs[1..], damaged_runs, damaged_runs_sum, memo);
    }

    memo.insert((springs.to_vec(), damaged_runs.to_vec()), num_arrangements);
    num_arrangements
}
