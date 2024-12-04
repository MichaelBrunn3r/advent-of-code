pub mod parse;
use std::ops::RangeInclusive;

use aoc::{ConstVec, Cursor};
use parse::{parse_parts, parse_workflows, Condition, OnMet, Part, Rule, Workflow, WF_IN_ID};

pub fn parse(input: &str) -> (&[Workflow; 1650], &[Rule], &[Part]) {
    let mut crs: Cursor<u8> = input.as_ptr().into();
    let (workflows, rules) = parse_workflows(&mut crs);
    crs.skip("\n".len());
    let parts = parse_parts(&mut crs);
    (workflows, rules, parts)
}

pub fn p1(workflows: &[(u16, u16)], rules: &[Rule], parts: &[Part]) -> usize {
    parts
        .iter()
        .filter(|part| {
            let mut current_workflow = &workflows[WF_IN_ID];
            loop {
                let rules = &rules[current_workflow.0 as usize..current_workflow.1 as usize];
                let matching_rule = rules.iter().find(|rule| rule.is_met(part)).unwrap();

                match matching_rule.on_met {
                    OnMet::Accept => return true,
                    OnMet::Reject => return false,
                    OnMet::Continue => {
                        current_workflow = &workflows[matching_rule.on_met_id as usize];
                    }
                }
            }
        })
        .map(|part| part.0.iter().sum::<u16>() as usize)
        .sum::<usize>()
}

static mut STACK: ConstVec<((u16, u16), [RangeInclusive<u16>; 4]), 16> =
    unsafe { std::mem::zeroed() };
static mut ACCEPTED: ConstVec<[RangeInclusive<u16>; 4], 600> = unsafe { std::mem::zeroed() };

pub fn p2(rules: &[Rule], workflows: &[(u16, u16); 1650]) -> usize {
    let stack = unsafe { &mut STACK };
    stack.clear();
    stack.push((
        workflows[WF_IN_ID],
        [1..=4000, 1..=4000, 1..=4000, 1..=4000],
    ));

    let accepted = unsafe { &mut ACCEPTED };
    accepted.clear();

    while let Some((workflow_id, mut xmas_ranges)) = stack.pop() {
        for rule in &rules[workflow_id.0 as usize..workflow_id.1 as usize] {
            match rule.condition {
                // Reject all
                Condition::LessThan(4001) => {}
                // Accept all
                Condition::LessThan(4002) => {
                    accepted.push(xmas_ranges.clone());
                }
                // Continue all
                Condition::LessThan(4003) => {
                    stack.push((workflows[rule.on_met_id as usize], xmas_ranges.clone()));
                }
                _ => {
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
