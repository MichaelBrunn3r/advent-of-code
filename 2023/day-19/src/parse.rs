use aoc::{StrExt, U8SliceExt};

use crate::{Condition, OnMet, Part, Rating, Rule};
use std::collections::HashMap;

pub struct WorkflowParser<'a> {
    pub data: &'a [u8],
    pub workflows: [(u16, u16); 1650],
    pub name_to_id: HashMap<&'a str, u16>,
    current_workflow: (&'a str, usize),
    num_rules: usize,
}

impl<'p> WorkflowParser<'p> {
    pub fn new(data: &[u8]) -> WorkflowParser {
        let mut parser = WorkflowParser {
            data,
            name_to_id: HashMap::with_capacity(1650),
            workflows: [(0, 0); 1650],
            current_workflow: ("", 0),
            num_rules: 0,
        };

        let name = data[..parser._find_rules_separator()].as_str_unchecked();
        parser.data = &parser.data[name.len() + 1..]; // Skip name and '{'
        parser.current_workflow = (name, parser.num_rules);

        parser
    }

    fn _name_to_id(&mut self, name: &'p str) -> u16 {
        if let Some(idx) = self.name_to_id.get(name) {
            return *idx;
        }

        let idx = self.name_to_id.len();
        self.name_to_id.insert(name, idx as u16);
        idx as u16
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
    type Item = Rule;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data[0] == b'\n' {
            return None;
        }

        self.num_rules += 1;
        if self.data[1] != b'<' && self.data[1] != b'>' {
            let rule = match self.data[0] {
                b'R' => {
                    self.data = &self.data[3..]; // Skip '}\n'
                    Rule {
                        rating: Rating::Any,
                        condition: Condition::Any,
                        on_met: OnMet::Reject,
                        on_met_workflow: u16::MAX,
                    }
                }
                b'A' => {
                    self.data = &self.data[3..]; // Skip '}\n'
                    Rule {
                        rating: Rating::Any,
                        condition: Condition::Any,
                        on_met: OnMet::Accept,
                        on_met_workflow: u16::MAX,
                    }
                }
                _ => {
                    let rules_terminator = self._find_rules_terminator();
                    let on_met_id =
                        self._name_to_id(self.data[..rules_terminator].as_str_unchecked());
                    self.data = &self.data[rules_terminator + 2..]; // Skip '<on_met>}\n'
                    Rule {
                        rating: Rating::Any,
                        condition: Condition::Any,
                        on_met: OnMet::Continue,
                        on_met_workflow: on_met_id,
                    }
                }
            };

            let id = self._name_to_id(self.current_workflow.0);
            self.workflows[id as usize] = (self.current_workflow.1 as u16, self.num_rules as u16);

            if self.data[0] != b'\n' {
                let name = self.data[..self._find_rules_separator()].as_str_unchecked();
                self.data = &self.data[name.len() + 1..]; // Skip name and '{'
                self.current_workflow = (name, self.num_rules);
            }

            return Some(rule);
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
        let condition_value = self.data[..on_met_sep]
            .as_str_unchecked()
            .parse_unsigned_unchecked();
        self.data = &self.data[on_met_sep + 1..];

        let condition = match condition_type {
            b'<' => Condition::LessThan(condition_value),
            b'>' => Condition::GreaterThan(condition_value),
            _ => unreachable!("Invalid condition"),
        };

        let (on_met, on_met_id) = match self.data[0] {
            b'A' => {
                self.data = &self.data[2..];
                (OnMet::Accept, u16::MAX)
            }
            b'R' => {
                self.data = &self.data[2..];
                (OnMet::Reject, u16::MAX)
            }
            _ => {
                // Get name. Name lengths: [3: 310, 2: 229]
                let pos_comma = if self.data[3] == b',' { 3 } else { 2 };

                let on_met_id = self._name_to_id(self.data[..pos_comma].as_str_unchecked());
                self.data = &self.data[pos_comma + 1..];

                (OnMet::Continue, on_met_id)
            }
        };

        Some(Rule {
            rating,
            condition,
            on_met,
            on_met_workflow: on_met_id,
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
