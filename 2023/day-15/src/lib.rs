use std::arch::x86_64::{__m256i, _mm256_mullo_epi16, _mm256_set_epi16, _mm256_storeu_si256};

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    SplitIter::new(&input.as_bytes()[..input.len() - 1])
        .map(|s| hash(s) % 256)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut boxes = vec![Box::<7>::new(); 256];

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
        .filter(|(b, _)| !b.is_empty())
        .flat_map(|(b, box_num)| {
            b.lenses[0..b.len]
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
struct Box<const N: usize> {
    lenses: [(usize, u8); N],
    len: usize,
}

impl<const N: usize> Box<N> {
    const fn new() -> Self {
        Self {
            lenses: [(0, 0); N],
            len: 0,
        }
    }

    fn add_lens(&mut self, label_hash: usize, focal_len: u8) {
        if let Some(pos) = self.lenses.iter().position(|&(h, _)| h == label_hash) {
            self.lenses[pos].1 = focal_len;
        } else {
            self.lenses[self.len] = (label_hash, focal_len);
            self.len += 1;
        }
    }

    fn remove_lens(&mut self, label_hash: usize) {
        if let Some(pos) = self.lenses.iter().position(|&(h, _)| h == label_hash) {
            for i in pos..self.len {
                self.lenses[i] = self.lenses[i + 1];
            }
            self.len -= 1;
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len == 0
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
            0..=2 => return None,
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

pub fn part_2_avx(input: &str) -> usize {
    let mut boxes = vec![Box::<7>::new(); 256];

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

    let data = boxes
        .iter()
        .zip(1..boxes.len() + 1)
        .filter(|(b, _)| !b.is_empty())
        .flat_map(|(b, box_num)| {
            b.lenses[0..b.len]
                .iter()
                .zip(1..b.lenses.len() + 1)
                .map(move |(&(_, focal_len), pos)| [box_num, pos, focal_len as usize])
        })
        .collect_vec();

    let iter = data.chunks_exact(16);

    let mut sum = iter
        .remainder()
        .iter()
        .map(|[box_num, pos, focal_len]| box_num * pos * *focal_len)
        .sum::<usize>();

    sum += iter
        .map(|chunk| {
            let box_nums = chunk_to_m256i(0, chunk);
            let lens_pos = chunk_to_m256i(1, chunk);
            let focal_lens = chunk_to_m256i(2, chunk);

            let res = unsafe { _mm256_mullo_epi16(box_nums, lens_pos) };
            let res = unsafe { _mm256_mullo_epi16(res, focal_lens) };

            let mut result_array: [i16; 16] = unsafe { std::mem::zeroed() };
            unsafe {
                _mm256_storeu_si256(result_array.as_mut_ptr() as *mut __m256i, res);
            }

            result_array.iter().sum::<i16>() as usize
        })
        .sum::<usize>();

    sum
}

fn chunk_to_m256i(idx: usize, chunk: &[[usize; 3]]) -> __m256i {
    unsafe {
        _mm256_set_epi16(
            chunk[15][idx] as i16,
            chunk[14][idx] as i16,
            chunk[13][idx] as i16,
            chunk[12][idx] as i16,
            chunk[11][idx] as i16,
            chunk[10][idx] as i16,
            chunk[9][idx] as i16,
            chunk[8][idx] as i16,
            chunk[7][idx] as i16,
            chunk[6][idx] as i16,
            chunk[5][idx] as i16,
            chunk[4][idx] as i16,
            chunk[3][idx] as i16,
            chunk[2][idx] as i16,
            chunk[1][idx] as i16,
            chunk[0][idx] as i16,
        )
    }
}
