use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let p = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for capture in p.captures_iter(input) {
        let a = capture.get(1).unwrap().as_str().as_bytes().parse_ascii_digits();
        let b = capture.get(2).unwrap().as_str().as_bytes().parse_ascii_digits();

        sum += a * b;
    }

    sum
}

pub fn part_2(input: &str) -> usize {
    let p = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<a>\d+),(?<b>\d+)\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for capture in p.captures_iter(input) {
        if capture.name("dont") != None {
            enabled = false;
        } else if capture.name("do") != None {
            enabled = true;
        } else if enabled {
            let a = capture.name("a").unwrap().as_str().as_bytes().parse_ascii_digits();
            let b = capture.name("b").unwrap().as_str().as_bytes().parse_ascii_digits();
            sum += a * b;
        }
    }
    
    sum
}
