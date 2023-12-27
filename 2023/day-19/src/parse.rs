use aoc::{StrExt, U8SliceExt};

use crate::{Condition, OnMet, Part, Rating, Rule};
use core::num;
use std::collections::HashMap;

pub struct WorkflowParser<'a> {
    pub data: &'a [u8],
    pub workflows: HashMap<&'a str, (usize, usize)>,
    current_workflow: (&'a str, usize),
    num_rules: usize,
}

impl WorkflowParser<'_> {
    pub fn new(data: &[u8]) -> WorkflowParser {
        let mut parser = WorkflowParser {
            data,
            workflows: HashMap::new(),
            current_workflow: ("", 0),
            num_rules: 0,
        };

        let name = data[..parser._find_rules_separator()].as_str_unchecked();
        parser.data = &parser.data[name.len() + 1..]; // Skip name and '{'
        parser.current_workflow = (name, parser.num_rules);

        parser
    }

    #[inline(always)]
    fn _next_byte_unchecked(&mut self) -> u8 {
        let byte = self.data[0];
        self.data = &self.data[1..];
        byte
    }

    #[inline(always)]
    fn _find_rules_separator(&mut self) -> usize {
        // Name lengths: [3: 310, 2: 229]
        if self.data[3] == b'{' {
            3
        } else {
            2
        }
    }

    fn _find_rules_terminator(&mut self) -> usize {
        // Name lengths: [3: 310, 2: 229]
        if self.data[3] == b'}' {
            3
        } else {
            2
        }
    }

    #[inline(always)]
    fn _find_on_met_separator(&mut self) -> usize {
        // Condition digits: [4: 728, 3: 313, 2: 19]
        if self.data[4] == b':' {
            4
        } else if self.data[3] == b':' {
            3
        } else {
            2
        }
    }
}

impl<'a> Iterator for WorkflowParser<'a> {
    type Item = Rule<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data[0] == b'\n' {
            return None;
        }

        self.num_rules += 1;
        if self.data[1] != b'<' && self.data[1] != b'>' {
            let rule = match self.data[0] {
                b'R' => {
                    self.data = &self.data[3..]; // Skip '}\n'
                    Some(Rule {
                        rating: Rating::Any,
                        condition: Condition::Any,
                        on_met: OnMet::Reject,
                    })
                }
                b'A' => {
                    self.data = &self.data[3..]; // Skip '}\n'
                    Some(Rule {
                        rating: Rating::Any,
                        condition: Condition::Any,
                        on_met: OnMet::Accept,
                    })
                }
                _ => {
                    let rules_terminator = self._find_rules_terminator();
                    let on_met = OnMet::Continue(self.data[..rules_terminator].as_str_unchecked());
                    self.data = &self.data[rules_terminator + 2..]; // Skip '<on_met>}\n'
                    Some(Rule {
                        rating: Rating::Any,
                        condition: Condition::Any,
                        on_met,
                    })
                }
            };

            self.workflows.insert(
                self.current_workflow.0,
                (self.current_workflow.1, self.num_rules),
            );

            if self.data[0] != b'\n' {
                let name = self.data[..self._find_rules_separator()].as_str_unchecked();
                self.data = &self.data[name.len() + 1..]; // Skip name and '{'
                self.current_workflow = (name, self.num_rules);
            }

            return rule;
        }

        let rating = match self._next_byte_unchecked() {
            b'x' => Rating::X,
            b'm' => Rating::M,
            b'a' => Rating::A,
            b's' => Rating::S,
            _ => unreachable!("Invalid rating"),
        };

        let condition_type = self._next_byte_unchecked();

        let on_met_sep = self._find_on_met_separator();
        let condition_value: usize = self.data[..on_met_sep]
            .as_str_unchecked()
            .parse_unsigned_unchecked();
        self.data = &self.data[on_met_sep + 1..];

        let condition = match condition_type {
            b'<' => Condition::LessThan(condition_value),
            b'>' => Condition::GreaterThan(condition_value),
            _ => unreachable!("Invalid condition"),
        };

        let on_met = match self.data[0] {
            b'A' => {
                self.data = &self.data[2..];
                OnMet::Accept
            }
            b'R' => {
                self.data = &self.data[2..];
                OnMet::Reject
            }
            _ => {
                // Get name. Name lengths: [3: 310, 2: 229]
                let pos_comma = if self.data[3] == b',' { 3 } else { 2 };

                let on_met_workflow = self.data[..pos_comma].as_str_unchecked();
                self.data = &self.data[pos_comma + 1..];

                OnMet::Continue(on_met_workflow)
            }
        };

        Some(Rule {
            rating,
            condition,
            on_met,
        })
    }
}

pub struct PartParser<'a> {
    data: &'a [u8],
}

impl PartParser<'_> {
    pub fn new(data: &[u8]) -> PartParser {
        PartParser { data }
    }

    #[inline(always)]
    fn _find_rating_separator(&mut self) -> usize {
        if self.data[6] == b',' {
            6
        } else if self.data[5] == b',' {
            5
        } else if self.data[4] == b',' {
            4
        } else {
            3
        }
    }

    #[inline(always)]
    fn _find_part_terminator(&mut self) -> usize {
        if self.data[6] == b'}' {
            6
        } else if self.data[5] == b'}' {
            5
        } else if self.data[4] == b'}' {
            4
        } else {
            3
        }
    }
}

impl<'a> Iterator for PartParser<'a> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            return None;
        }

        self.data = &self.data[1..]; // Skip '{'

        let mut part = Part([0, 0, 0, 0]);

        for i in 0..3 {
            let rating_sep = self._find_rating_separator();
            part.0[i] = self.data[2..rating_sep]
                .as_str_unchecked()
                .parse_unsigned_unchecked();
            self.data = &self.data[rating_sep + 1..];
        }

        let part_terminator = self._find_part_terminator();
        part.0[3] = self.data[2..part_terminator]
            .as_str_unchecked()
            .parse_unsigned_unchecked();

        self.data = &self.data[part_terminator + 2..]; // Skip '}\n'

        Some(part)
    }
}
