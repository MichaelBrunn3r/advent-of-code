use std::{collections::HashMap, str::FromStr};

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, conditions) = line[0..line.len() - 1].split_once('{').unwrap();
            let conditions = conditions.split(',').map(Rule::parse_str).collect_vec();

            (name, conditions)
        })
        .collect::<HashMap<&str, Vec<Rule>>>();
    let first_workflow = workflows.get("in").unwrap();

    let parts = parts.lines().map(Part::from).collect_vec();

    parts
        .iter()
        .filter(|part| {
            let mut current_workflow = first_workflow;
            loop {
                match apply_workflow(&current_workflow, &part) {
                    OnMet::Accept => return true,
                    OnMet::Reject => return false,
                    OnMet::Continue(workflow) => {
                        current_workflow = workflows.get(workflow).unwrap();
                    }
                }
            }
        })
        .map(|part| part.total_rating())
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    0
}

fn apply_workflow<'a>(workflow: &'a [Rule], part: &Part) -> &'a OnMet<'a> {
    &workflow
        .iter()
        .find(|rule| rule.is_met(part))
        .unwrap()
        .on_met
}

#[derive(Debug)]
struct Rule<'a> {
    rating: Rating,
    condition: Condition,
    on_met: OnMet<'a>,
}

impl<'a> Rule<'a> {
    fn parse_str(s: &'a str) -> Self {
        if let Some((condition, on_met)) = s.split_once(':') {
            let on_met = match on_met {
                "A" => OnMet::Accept,
                "R" => OnMet::Reject,
                _ => OnMet::Continue(on_met),
            };

            let category = match condition.as_bytes()[0] {
                b'x' => Rating::X,
                b'm' => Rating::M,
                b'a' => Rating::A,
                b's' => Rating::S,
                _ => Rating::Any,
            };

            let condition_value = condition[2..].parse::<usize>().unwrap();

            let condition = match s.as_bytes()[1] {
                b'<' => Condition::LessThan(condition_value),
                b'>' => Condition::GreaterThan(condition_value),
                _ => Condition::Any,
            };

            Self {
                rating: category,
                condition,
                on_met,
            }
        } else {
            let on_met = match s {
                "A" => OnMet::Accept,
                "R" => OnMet::Reject,
                _ => OnMet::Continue(s),
            };

            Self {
                rating: Rating::Any,
                condition: Condition::Any,
                on_met,
            }
        }
    }

    fn is_met(&self, part: &Part) -> bool {
        match self.rating {
            Rating::X => self.condition.is_met(part.x),
            Rating::M => self.condition.is_met(part.m),
            Rating::A => self.condition.is_met(part.a),
            Rating::S => self.condition.is_met(part.s),
            Rating::Any => true,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Rating {
    X,
    M,
    A,
    S,
    Any,
}

#[derive(Debug)]
enum Condition {
    LessThan(usize),
    GreaterThan(usize),
    Any,
}

impl Condition {
    fn is_met(&self, value: usize) -> bool {
        match self {
            Self::LessThan(n) => value < *n,
            Self::GreaterThan(n) => value > *n,
            Self::Any => true,
        }
    }
}

#[derive(Debug)]
enum OnMet<'a> {
    Accept,
    Reject,
    Continue(&'a str),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(line: &str) -> Self {
        let mut parts = line[1..line.len() - 1]
            .split(',')
            .map(|part| part.split_once('=').unwrap().1.parse::<usize>().unwrap());

        let x = parts.next().unwrap();
        let m = parts.next().unwrap();
        let a = parts.next().unwrap();
        let s = parts.next().unwrap();

        Self { x, m, a, s }
    }
}
