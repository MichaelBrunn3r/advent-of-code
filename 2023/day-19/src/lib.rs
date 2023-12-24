#![allow(unused_imports, unused_variables)]

mod parse;

use aoc::prelude::*;
use itertools::Itertools;
use parse::{PartParser, WorkflowParser};
use regex::Regex;
use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

pub fn part_1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = WorkflowParser::new(input.as_bytes()).collect::<HashMap<&str, Vec<Rule>>>();
    let first_workflow = workflows.get("in").unwrap();

    PartParser::new(parts.as_bytes())
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
        .map(|part| part.0.iter().sum::<usize>())
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    let workflows = WorkflowParser::new(input.as_bytes()).collect::<HashMap<&str, Vec<Rule>>>();

    let mut stack = vec![(
        workflows.get("in").unwrap(),
        [1..=4000, 1..=4000, 1..=4000, 1..=4000],
    )];
    let mut accepted = vec![];

    while !stack.is_empty() {
        let (workflow, mut xmas_ranges) = stack.pop().unwrap();

        for rule in workflow {
            match rule.rating {
                Rating::Any => match rule.on_met {
                    OnMet::Accept => {
                        accepted.push(xmas_ranges.clone());
                    }
                    OnMet::Continue(workflow) => {
                        stack.push((workflows.get(workflow).unwrap(), xmas_ranges.clone()));
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
                            stack.push((workflows.get(workflow).unwrap(), new_ranges));
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

fn apply_workflow<'a>(workflow: &'a [Rule], part: &Part) -> &'a OnMet<'a> {
    &workflow
        .iter()
        .find(|rule| rule.is_met(part))
        .unwrap()
        .on_met
}

#[derive(Debug)]
pub struct Rule<'a> {
    rating: Rating,
    condition: Condition,
    on_met: OnMet<'a>,
}

impl<'a> Rule<'a> {
    fn is_met(&self, part: &Part) -> bool {
        match self.rating {
            Rating::X => self.condition.is_met(part.x()),
            Rating::M => self.condition.is_met(part.m()),
            Rating::A => self.condition.is_met(part.a()),
            Rating::S => self.condition.is_met(part.s()),
            Rating::Any => true,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
    Any,
}

#[derive(Debug)]
pub enum Condition {
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
pub enum OnMet<'a> {
    Accept,
    Reject,
    Continue(&'a str),
}

#[derive(Debug)]
pub struct Part([usize; 4]);

impl Part {
    #[inline(always)]
    fn x(&self) -> usize {
        self.0[0]
    }

    #[inline(always)]
    fn m(&self) -> usize {
        self.0[1]
    }

    #[inline(always)]
    fn a(&self) -> usize {
        self.0[2]
    }

    #[inline(always)]
    fn s(&self) -> usize {
        self.0[3]
    }
}
