use aoc::{StrExt, U8SliceExt};

use crate::{Condition, OnMet, Part, Rating, Rule};

pub fn parse_workflow(line: &[u8]) -> (&str, Vec<Rule>) {
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

pub fn parse_part(mut line: &[u8]) -> Part {
    // Remove brackets
    line = &line[1..line.len() - 1];

    let pos_comma = if line[6] == b',' {
        6
    } else if line[5] == b',' {
        5
    } else if line[4] == b',' {
        4
    } else {
        3
    };
    let x = line[2..pos_comma]
        .as_str_unchecked()
        .parse_unsigned_unchecked();
    line = &line[pos_comma + 1..];

    let pos_comma = if line[6] == b',' {
        6
    } else if line[5] == b',' {
        5
    } else if line[4] == b',' {
        4
    } else {
        3
    };
    let m = line[2..pos_comma]
        .as_str_unchecked()
        .parse_unsigned_unchecked();
    line = &line[pos_comma + 1..];

    let pos_comma = if line[6] == b',' {
        6
    } else if line[5] == b',' {
        5
    } else if line[4] == b',' {
        4
    } else {
        3
    };
    let a = line[2..pos_comma]
        .as_str_unchecked()
        .parse_unsigned_unchecked();
    line = &line[pos_comma + 1..];

    let s = line[2..line.len()]
        .as_str_unchecked()
        .parse_unsigned_unchecked();

    Part { x, m, a, s }
}
