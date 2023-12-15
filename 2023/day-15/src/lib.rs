use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    // SplitIter::new(&input.as_bytes()[..input.len() - 1]).for_each(|step| {
    //     println!("{:?}", step.as_str_unchecked());
    // });

    // println!("{:?}", input.split(',').map(|step| step.len()).counts());

    SplitIter::new(&input.as_bytes()[..input.len() - 1])
        .map(|step| {
            step.into_iter()
                .fold(0u8, |acc, &c| (acc.wrapping_add(c)).wrapping_mul(17)) as usize
        })
        .sum()
    // 0
}

pub fn part_2(input: &str) -> usize {
    0
}

struct SplitIter<'i> {
    input: &'i [u8],
    pos: usize,
}

impl<'i> SplitIter<'i> {
    fn new(input: &'i [u8]) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'i> Iterator for SplitIter<'i> {
    type Item = &'i [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let chars_left = self.input.len().saturating_sub(self.pos);
        match chars_left {
            0 | 1 | 2 => return None,
            3 => {
                let result = &self.input[self.pos..self.pos + 3];
                self.pos += 3;
                return Some(result);
            }
            _ => {}
        }

        // 4: 1341, 5: 1160, 3: 580, 6: 531, 7: 263, 8: 125

        // 4 characters
        if chars_left >= 5 && self.input[self.pos + 4] == b',' {
            let result = &self.input[self.pos..self.pos + 4];
            self.pos += 5;
            return Some(result);
        }
        // 5 characters
        if chars_left >= 6 && self.input[self.pos + 5] == b',' {
            let result = &self.input[self.pos..self.pos + 5];
            self.pos += 6;
            return Some(result);
        }
        // 3 characters
        if chars_left >= 4 && self.input[self.pos + 3] == b',' {
            let result = &self.input[self.pos..self.pos + 3];
            self.pos += 4;
            return Some(result);
        }
        // 6 characters
        if chars_left >= 7 && self.input[self.pos + 6] == b',' {
            let result = &self.input[self.pos..self.pos + 6];
            self.pos += 7;
            return Some(result);
        }
        // 7 characters
        if chars_left >= 8 && self.input[self.pos + 7] == b',' {
            let result = &self.input[self.pos..self.pos + 7];
            self.pos += 8;
            return Some(result);
        }
        // 8 characters
        if chars_left >= 9 && self.input[self.pos + 8] == b',' {
            let result = &self.input[self.pos..self.pos + 8];
            self.pos += 9;
            return Some(result);
        }

        panic!("No match found");
    }
}
