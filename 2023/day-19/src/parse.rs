use aoc::{StrExt, U8SliceExt};

use crate::{Condition, OnMet, Part, Rating, Rule};

pub struct WorkflowParser<'a> {
    pub data: &'a [u8],
}

impl WorkflowParser<'_> {
    pub fn new(data: &[u8]) -> WorkflowParser {
        WorkflowParser { data }
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
    type Item = (&'a str, Vec<Rule<'a>>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.data[0] == b'\n' {
            return None;
        }

        let name = self.data[..self._find_rules_separator()].as_str_unchecked();
        self.data = &self.data[name.len() + 1..]; // Skip name and '{'

        let mut rules = vec![];

        loop {
            if self.data[1] != b'<' && self.data[1] != b'>' {
                match self.data[0] {
                    b'R' => {
                        rules.push(Rule {
                            rating: Rating::Any,
                            condition: Condition::Any,
                            on_met: OnMet::Reject,
                        });
                        self.data = &self.data[3..]; // Skip '}\n'
                    }
                    b'A' => {
                        rules.push(Rule {
                            rating: Rating::Any,
                            condition: Condition::Any,
                            on_met: OnMet::Accept,
                        });
                        self.data = &self.data[3..]; // Skip '}\n'
                    }
                    _ => {
                        let rules_terminator = self._find_rules_terminator();
                        rules.push(Rule {
                            rating: Rating::Any,
                            condition: Condition::Any,
                            on_met: OnMet::Continue(
                                self.data[..rules_terminator].as_str_unchecked(),
                            ),
                        });
                        self.data = &self.data[rules_terminator + 2..]; // Skip '<on_met>}\n'
                    }
                }
                break;
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

            rules.push(Rule {
                rating,
                condition,
                on_met,
            });
        }

        Some((name, rules))
    }
}

pub fn parse_part(mut line: &[u8]) -> Part {
    // Remove brackets
    line = &line[1..line.len() - 1];

    let mut part = Part([0, 0, 0, 0]);

    for i in 0..3 {
        let pos_comma = if line[6] == b',' {
            6
        } else if line[5] == b',' {
            5
        } else if line[4] == b',' {
            4
        } else {
            3
        };
        part.0[i] = line[2..pos_comma]
            .as_str_unchecked()
            .parse_unsigned_unchecked();
        line = &line[pos_comma + 1..];
    }

    part.0[3] = line[2..line.len()]
        .as_str_unchecked()
        .parse_unsigned_unchecked();

    part
}
