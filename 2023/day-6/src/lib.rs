use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

// distance = hold_time * (time - hold_time)
// -> distance_to_beat < hold_time * (time - hold_time)

// d = h * (t - h) (h=1.7)
// d/h = t-h
// d/h + h = t
// d + x^2 = th
// h^2 - th + d = 0

pub fn part_1(input: &str) -> usize {
    let re_number = Regex::new(r"(\d+)").unwrap();
    let (times, distances_to_beat) = input
        .split('\n')
        .map(|line| {
            re_number
                .find_iter(line.split_once(':').unwrap().1)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect_tuple()
        .unwrap();

    times
        .iter()
        .zip(distances_to_beat.iter())
        .map(|(time, distance_to_beat)| {
            let (max_hold_time, min_hold_time) =
                quadratic_formula(1.0, -(*time as f64), *distance_to_beat as f64);

            (
                (max_hold_time - 1.0).ceil() as usize,
                (min_hold_time + 1.0).floor() as usize,
            )
        })
        .map(|(max_hold_time, min_hold_time)| max_hold_time - min_hold_time + 1)
        .reduce(|acc, x| acc * x)
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let (time, distance_to_beat) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .replace(" ", "")
                .parse::<usize>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    // println!("time: {}, distance_to_beat: {}", time, distance_to_beat);

    let (max_hold_time, min_hold_time) =
        quadratic_formula(1.0, -(time as f64), distance_to_beat as f64);

    ((max_hold_time - 1.0).ceil() as usize) - ((min_hold_time + 1.0).floor() as usize) + 1
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + root) / 2.0 * a;
    let x2 = (-b - root) / 2.0 * a;

    (x1, x2)
}

fn calculate_distance(time: usize, hold_time: usize) -> usize {
    hold_time * (time - hold_time)
}
