use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

const OPERATIONAL: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKOWN: u8 = b'?';

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let damaged_runs: Vec<usize> = groups.split(',').map(|g| g.parse().unwrap()).collect();
            (springs.as_bytes(), damaged_runs)
        })
        .map(|(springs, damaged_runs)| count_arrangements(springs, &damaged_runs))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

fn count_arrangements(springs: &[u8], damaged_runs: &[usize]) -> usize {
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

    if springs.len() < damaged_runs.iter().sum::<usize>() + damaged_runs.len() - 1 {
        return 0;
    }

    if springs[0] == OPERATIONAL {
        return count_arrangements(&springs[1..], damaged_runs);
    }

    let mut num_arrangements = 0;
    let run = damaged_runs[0];
    let all_not_operational = springs[..run].iter().all(|c| *c != OPERATIONAL);
    let end = (run + 1).min(springs.len());

    if all_not_operational
        && ((springs.len() > run && springs[run] != DAMAGED) || springs.len() <= run)
    {
        num_arrangements = count_arrangements(&springs[end..], &damaged_runs[1..]);
    }

    if springs[0] == UNKOWN {
        num_arrangements += count_arrangements(&springs[1..], damaged_runs);
    }

    num_arrangements
}
