#![allow(unused_variables)]

mod parse;
use parse::{PartParser, WorkflowParser};

pub fn part_1(input: &str) -> usize {
    let mut parser = WorkflowParser::new(input.as_bytes());
    let mut rules = Vec::with_capacity(1650);
    while let Some(rule) = parser.next() {
        rules.push(rule)
    }

    let workflows = parser.workflows;
    let workflow_in_id = *parser.name_to_id.get("in").unwrap();

    PartParser::new(&parser.data[1..])
        .filter(|part| {
            let mut current_workflow = workflows[workflow_in_id as usize];
            loop {
                let rules = &rules[current_workflow.0 as usize..current_workflow.1 as usize];
                let matching_rule = rules.iter().find(|rule| rule.is_met(part)).unwrap();

                match matching_rule.on_met {
                    OnMet::Accept => return true,
                    OnMet::Reject => return false,
                    OnMet::Continue => {
                        current_workflow = workflows[matching_rule.on_met_workflow as usize];
                    }
                }
            }
        })
        .map(|part| part.0.iter().sum::<u16>() as usize)
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    let mut parser = WorkflowParser::new(input.as_bytes());
    let mut rules = Vec::with_capacity(1650);
    while let Some(rule) = parser.next() {
        rules.push(rule)
    }

    let workflows = parser.workflows;

    let mut stack = vec![(
        workflows[*parser.name_to_id.get("in").unwrap() as usize],
        [1..=4000, 1..=4000, 1..=4000, 1..=4000],
    )];
    let mut accepted = vec![];

    while !stack.is_empty() {
        let (workflow_id, mut xmas_ranges) = stack.pop().unwrap();

        for rule in &rules[workflow_id.0 as usize..workflow_id.1 as usize] {
            match rule.rating {
                Rating::Any => match rule.on_met {
                    OnMet::Accept => {
                        accepted.push(xmas_ranges.clone());
                    }
                    OnMet::Continue => {
                        stack.push((
                            workflows[rule.on_met_workflow as usize],
                            xmas_ranges.clone(),
                        ));
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
                        OnMet::Continue => {
                            stack.push((workflows[rule.on_met_workflow as usize], new_ranges));
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
    on_met_workflow: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum OnMet {
    Accept,
    Reject,
    Continue,
}

impl Rule {
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

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    LessThan(u16),
    GreaterThan(u16),
    Any,
}

impl Condition {
    fn is_met(&self, value: u16) -> bool {
        match self {
            Self::LessThan(n) => value < *n,
            Self::GreaterThan(n) => value > *n,
            Self::Any => true,
        }
    }
}

#[derive(Debug)]
pub struct Part([u16; 4]);

impl Part {
    #[inline(always)]
    fn x(&self) -> u16 {
        self.0[0]
    }

    #[inline(always)]
    fn m(&self) -> u16 {
        self.0[1]
    }

    #[inline(always)]
    fn a(&self) -> u16 {
        self.0[2]
    }

    #[inline(always)]
    fn s(&self) -> u16 {
        self.0[3]
    }
}
