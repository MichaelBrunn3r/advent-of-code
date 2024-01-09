#![allow(unused_variables)]

mod parse;
use std::collections::HashMap;

use fxhash::FxHashMap;
use itertools::Itertools;
use parse::{PartParser, WorkflowParser};

pub fn parse<'a>(
    input: &'a str,
) -> (
    Vec<Rule>,
    [(u16, u16); 1650],
    u16,
    Vec<Part>,
    FxHashMap<&'a str, u16>,
) {
    let mut parser = WorkflowParser::new(input.as_bytes());
    let mut rules = Vec::with_capacity(1650);
    for rule in parser.by_ref() {
        rules.push(rule)
    }

    let workflows = parser.workflows;
    let workflow_in_id = *parser.name_to_id.get("in").unwrap();

    let parts = PartParser::new(&parser.data[1..]).collect_vec();

    (rules, workflows, workflow_in_id, parts, parser.name_to_id)
}

pub fn part_1(
    rules: &[Rule],
    workflows: &[(u16, u16); 1650],
    workflow_in_id: u16,
    parts: &[Part],
) -> usize {
    parts
        .iter()
        .filter(|part| {
            let mut current_workflow = workflows[workflow_in_id as usize];
            loop {
                let rules = &rules[current_workflow.0 as usize..current_workflow.1 as usize];
                let matching_rule = rules.iter().find(|rule| rule.is_met(part)).unwrap();

                match matching_rule.on_met {
                    OnMet::Accept => return true,
                    OnMet::Reject => return false,
                    OnMet::Continue => {
                        current_workflow = workflows[matching_rule.on_met_id as usize];
                    }
                }
            }
        })
        .map(|part| part.0.iter().sum::<u16>() as usize)
        .sum::<usize>()
}

pub fn part_2(
    rules: &[Rule],
    workflows: &[(u16, u16); 1650],
    name_to_id: &FxHashMap<&str, u16>,
) -> usize {
    let mut stack = vec![(
        workflows[*name_to_id.get("in").unwrap() as usize],
        [1..=4000, 1..=4000, 1..=4000, 1..=4000],
    )];
    let mut accepted = vec![];

    while let Some((workflow_id, mut xmas_ranges)) = stack.pop() {
        for rule in &rules[workflow_id.0 as usize..workflow_id.1 as usize] {
            match rule.rating {
                Rating::Any => match rule.on_met {
                    OnMet::Accept => {
                        accepted.push(xmas_ranges.clone());
                    }
                    OnMet::Continue => {
                        stack.push((workflows[rule.on_met_id as usize], xmas_ranges.clone()));
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
                    }

                    match rule.on_met {
                        OnMet::Accept => {
                            accepted.push(new_ranges);
                        }
                        OnMet::Continue => {
                            stack.push((workflows[rule.on_met_id as usize], new_ranges));
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
                .map(|range| (range.end() - range.start() + 1) as usize)
                .product::<usize>()
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    rating: Rating,
    condition: Condition,
    on_met: OnMet,
    on_met_id: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum OnMet {
    Accept,
    Reject,
    Continue,
}

impl Rule {
    fn is_met(&self, part: &Part) -> bool {
        self.condition.is_met(part.0[self.rating as usize & 0b11])
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
    Any = 4,
}

impl Rating {
    #[inline(always)]
    fn from_ascii_char(c: u8) -> Self {
        const LUT: [Rating; 121] = Rating::_create_lut();
        LUT[c as usize]
    }

    const fn _create_lut() -> [Rating; 121] {
        let mut lut = [Rating::Any; 121];
        lut[b'x' as usize] = Rating::X;
        lut[b'm' as usize] = Rating::M;
        lut[b'a' as usize] = Rating::A;
        lut[b's' as usize] = Rating::S;
        lut
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Condition {
    LessThan(u16) = b'<',
    GreaterThan(u16) = b'>',
}

impl Condition {
    #[inline(always)]
    fn from_ascii_char(c: u8, value: u32) -> Self {
        unsafe { std::mem::transmute(c as u32 | value << 16) }
    }

    fn is_met(&self, value: u16) -> bool {
        match self {
            Self::LessThan(n) => value < *n,
            Self::GreaterThan(n) => value > *n,
        }
    }
}

#[derive(Debug)]
pub struct Part([u16; 4]);
