use aoc::prelude::*;
use itertools::Itertools;

const NUM_LINES_RULES: usize = 1176;
const LINE_LENGTH_RULES: usize = "12|34\n".len(); // ASSUMPTION 1: All rules match the pattern '\d\d\|\d\d\n'
const NUM_LINES_UPDATES: usize = 191;
const NUM_PAGES: usize = 90;
type LUT = [bool;NUM_PAGES];

pub fn parse(input: &str) -> (Vec<LUT>, Vec<Vec<u8>>) {
    let bytes = input.as_bytes();

    let mut rules = vec![[false; NUM_PAGES]; NUM_PAGES];
    bytes
        .chunks_exact(LINE_LENGTH_RULES)
        .take(NUM_LINES_RULES)
        .for_each(|l| {
            let a = l.parse_n_ascii_digits(2) as u8;
            let b = l[3..].parse_n_ascii_digits(2) as u8;
            rules[page_to_idx(b)][page_to_idx(a)] = true;
        });

    let updates = bytes[(LINE_LENGTH_RULES * NUM_LINES_RULES)+1..]
        .split(|&b| b == b'\n')
        .take(NUM_LINES_UPDATES)
        .map(|mut l| {
            let mut pages = vec![l.parse_n_ascii_digits(2) as u8];
            l = &l[2..];

            while l.len() >= 2 {
                l = &l[1..];
                pages.push(l.parse_n_ascii_digits(2) as u8);
                l = &l[2..];
            }

            pages
        })
        .collect_vec();

    (rules, updates)
}

pub fn p1(rules: &[LUT], updates: &[Vec<u8>]) -> usize {
    updates
        .iter()
        .filter_map(|pages| {
            if !is_correctly_ordered(&pages, &rules) {
                return None
            }
            Some(pages[pages.len()/2] as usize)
        })
        .sum()
}

// 11           = 2  = 2*1 + (1-1)
// 11,22        = 5  = 2*2 + (2-1)
// 11,22,33     = 8  = 3*2 + (3-1)
// 11,22,33,44  = 11 = 4*2 + (4-1)

pub fn p2(rules: &[LUT], updates: Vec<Vec<u8>>) -> usize {
    updates
        .into_iter()
        .filter(|pages| !is_correctly_ordered(pages, &rules))
        .map(|mut pages| {
            for i in 1..pages.len() {
                let page = pages[i];
                for il in (0..i).rev() {
                    let before_il = &rules[page_to_idx(pages[il])];
                    if !before_il[page_to_idx(page)] {
                        break;
                    }

                    pages[il+1] = pages[il];
                    pages[il] = page;
                }
            }

            pages[pages.len()/2] as usize
        })
        .sum()
}

// ASSUMPTION 2: Page numbers range from 10-99
fn page_to_idx(page: u8) -> usize {
    (page - 10) as usize
}

fn is_correctly_ordered(pages: &[u8], rules: &[LUT]) -> bool {
    for i in 0..pages.len() {
        let before_i = &rules[page_to_idx(pages[i])];
        for ir in i+1..pages.len() {
            if before_i[page_to_idx(pages[ir])] {
                return false;
            }
        }
    }

    true
}
