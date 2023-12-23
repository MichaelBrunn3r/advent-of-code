#![allow(unused_imports, unused_variables)]

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

pub fn part_1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| parse_workflow(&line.as_bytes()))
        .collect::<HashMap<&str, Vec<Rule>>>();
    let first_workflow = workflows.get("in").unwrap();

    parts
        .lines()
        .map(Part::from)
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
    let workflows = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| parse_workflow(&line.as_bytes()))
        .collect::<HashMap<&str, Vec<Rule>>>();

    let mut stack = vec![("in", [1..=4000, 1..=4000, 1..=4000, 1..=4000])];
    let mut accepted = vec![];

    while !stack.is_empty() {
        let (name, mut xmas_ranges) = stack.pop().unwrap();
        let workflow = workflows.get(name).unwrap();

        for rule in workflow {
            match rule.rating {
                Rating::Any => match rule.on_met {
                    OnMet::Accept => {
                        accepted.push(xmas_ranges.clone());
                    }
                    OnMet::Continue(workflow) => {
                        stack.push((workflow, xmas_ranges.clone()));
                    }
                    _ => {}
                },
                Rating::X | Rating::M | Rating::A | Rating::S => {
                    let rating_idx = rule.rating as usize;

                    let mut new_ranges = xmas_ranges.clone();
                    match rule.condition {
                        Condition::LessThan(n) => {
                            new_ranges[rating_idx] = *new_ranges[rating_idx].start()..=n - 1;
                            xmas_ranges[rating_idx] = n..=*xmas_ranges[rating_idx].end();
                        }
                        Condition::GreaterThan(n) => {
                            new_ranges[rating_idx] = n + 1..=*new_ranges[rating_idx].end();
                            xmas_ranges[rating_idx] = *xmas_ranges[rating_idx].start()..=n;
                        }
                        _ => {}
                    }

                    match rule.on_met {
                        OnMet::Accept => {
                            accepted.push(new_ranges);
                        }
                        OnMet::Continue(workflow) => {
                            stack.push((workflow, new_ranges));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    accepted
        .iter()
        .map(|xmas_ranges| {
            xmas_ranges
                .iter()
                .map(|range| range.end() - range.start() + 1)
                .product::<usize>()
        })
        .sum()
}

fn parse_workflow(line: &[u8]) -> (&str, Vec<Rule>) {
    // Get name. Name lengths: [3: 310, 2: 229]
    let pos_opening_bracket = if line[3] == b'{' { 3 } else { 2 };
    let name = line[..pos_opening_bracket].as_str_unchecked();

    let mut rules = vec![];

    // Remove name and brackets ('abc{<rules>}' -> '<rules>')
    let mut rules_str = &line[pos_opening_bracket + 1..line.len() - 1];
    while rules_str.len() > 0 {
        if rules_str.len() <= 3 {
            rules.push(match rules_str[0] {
                b'A' => Rule {
                    rating: Rating::Any,
                    condition: Condition::Any,
                    on_met: OnMet::Accept,
                },
                b'R' => Rule {
                    rating: Rating::Any,
                    condition: Condition::Any,
                    on_met: OnMet::Reject,
                },
                _ => Rule {
                    rating: Rating::Any,
                    condition: Condition::Any,
                    on_met: OnMet::Continue(rules_str.as_str_unchecked()),
                },
            });
            break;
        }

        let rating = match rules_str[0] {
            b'x' => Rating::X,
            b'm' => Rating::M,
            b'a' => Rating::A,
            b's' => Rating::S,
            _ => unreachable!("Invalid rating"),
        };
        rules_str = &rules_str[1..];

        let condition_type = rules_str[0];
        rules_str = &rules_str[1..];

        // Condition digits: [4: 728, 3: 313, 2: 19]
        let pos_colon = if rules_str[4] == b':' {
            4
        } else if rules_str[3] == b':' {
            3
        } else {
            2
        };
        let condition_value = rules_str[..pos_colon]
            .as_str_unchecked()
            .parse_unsigned_unchecked();
        rules_str = &rules_str[pos_colon + 1..];

        let condition = match condition_type {
            b'<' => Condition::LessThan(condition_value),
            b'>' => Condition::GreaterThan(condition_value),
            _ => unreachable!("Invalid condition"),
        };

        let on_met = match rules_str[0] {
            b'A' => {
                rules_str = &rules_str[2..];
                OnMet::Accept
            }
            b'R' => {
                rules_str = &rules_str[2..];
                OnMet::Reject
            }
            _ => {
                // Get name. Name lengths: [3: 310, 2: 229]
                let pos_comma = if rules_str[3] == b',' { 3 } else { 2 };

                let on_met_workflow = rules_str[..pos_comma].as_str_unchecked();
                rules_str = &rules_str[pos_comma + 1..];

                OnMet::Continue(on_met_workflow)
            }
        };

        rules.push(Rule {
            rating,
            condition,
            on_met,
        });
    }

    (name, rules)
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
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
