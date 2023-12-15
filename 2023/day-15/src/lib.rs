use aoc::prelude::*;

pub fn part_1(input: &str) -> usize {
    SplitIter::new(&input.as_bytes()[..input.len() - 1])
        .map(|s| hash(s) % 256 as usize)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut boxes = vec![Box::new(); 256];

    SplitIter::new(&input.as_bytes()[..input.len() - 1]).for_each(|step| {
        let is_add_operation = step[step.len() - 1] != b'-';

        // '[a-z]+=\d': 2016, '[a-z]+-': 1984
        if is_add_operation {
            let label = &step[..step.len() - 2];
            let label_hash = hash(label);
            let focal_len = step[step.len() - 1] - b'0';
            boxes[label_hash % 256].add_lens(label_hash + label[0] as usize, focal_len);
        } else {
            let label = &step[..step.len() - 1];
            let label_hash = hash(label);
            boxes[label_hash % 256].remove_lens(label_hash + label[0] as usize);
        }
    });

    boxes
        .iter()
        .zip(1..boxes.len() + 1)
        .filter(|(b, _)| !b.lenses.is_empty())
        .flat_map(|(b, box_num)| {
            b.lenses
                .iter()
                .zip(1..b.lenses.len() + 1)
                .map(move |(&(_, focal_len), pos)| box_num * pos * focal_len as usize)
        })
        .sum()
}

#[inline(always)]
fn hash(input: &[u8]) -> usize {
    input
        .iter()
        .fold(0usize, |acc, &c| ((acc + c as usize) * 17))
}

#[derive(Debug, Clone)]
struct Box {
    lenses: Vec<(usize, u8)>,
}

impl Box {
    fn new() -> Self {
        Self {
            // Measures max 6 lenses per box
            lenses: Vec::with_capacity(6),
        }
    }

    fn add_lens(&mut self, label_hash: usize, focal_len: u8) {
        if let Some(pos) = self.lenses.iter().position(|&(h, _)| h == label_hash) {
            self.lenses[pos].1 = focal_len;
        } else {
            self.lenses.push((label_hash, focal_len));
        }
    }

    fn remove_lens(&mut self, label_hash: usize) {
        if let Some(pos) = self.lenses.iter().position(|&(h, _)| h == label_hash) {
            self.lenses.remove(pos);
        }
    }
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
            4 => {
                let result = &self.input[self.pos..self.pos + 4];
                self.pos += 4;
                return Some(result);
            }
            _ => {}
        }

        // 4: 1341, 5: 1160, 3: 580, 6: 531, 7: 263, 8: 125
        let comma_offset = if chars_left >= 5 && self.input[self.pos + 4] == b',' {
            4
        } else if chars_left >= 6 && self.input[self.pos + 5] == b',' {
            5
        } else if chars_left >= 4 && self.input[self.pos + 3] == b',' {
            3
        } else if chars_left >= 7 && self.input[self.pos + 6] == b',' {
            6
        } else if chars_left >= 8 && self.input[self.pos + 7] == b',' {
            7
        } else if chars_left >= 9 && self.input[self.pos + 8] == b',' {
            8
        } else {
            panic!("No match found");
        };

        let result = &self.input[self.pos..self.pos + comma_offset];
        self.pos += comma_offset + 1;
        Some(result)
    }
}
