use aoc::{prelude::*, ConstVec};
use itertools::Itertools;

const NUM_LINES_RULES: usize = 1176;
const LINE_LENGTH_RULES: usize = "12|34\n".len(); // ASSUMPTION 1: All rules match the pattern '\d\d\|\d\d\n'
const NUM_LINES_UPDATES: usize = 191;
const NUM_PAGES: usize = 90;

type Rules = [[bool;NUM_PAGES]; NUM_PAGES];
type Update = Vec<u8>;

static mut RULES: [[bool; NUM_PAGES]; NUM_PAGES] = unsafe{std::mem::zeroed()};

pub fn parse(input: &str) -> (&Rules, Vec<Update>, Vec<Update>) {
    let bytes = input.as_bytes();
    unsafe{RULES = std::mem::zeroed()};

    bytes
        .chunks_exact(LINE_LENGTH_RULES)
        .take(NUM_LINES_RULES)
        .for_each(|l| {
            let a = l.parse_n_ascii_digits(2) as u8;
            let b = l[3..].parse_n_ascii_digits(2) as u8;
            unsafe{RULES[page_to_idx(b)][page_to_idx(a)] = true;}
        });

    let mut correct_updates = Vec::new();
    let mut wrong_updates = Vec::new();

    let mut crs = &bytes[(LINE_LENGTH_RULES * NUM_LINES_RULES)+1..];
    for i in 0..NUM_LINES_UPDATES {
        let mut pages = Vec::new();

        loop {
            pages.push(crs.parse_n_ascii_digits(2) as u8);
            crs = &crs[2..]; // Skip page number

            let c= crs[0];
            crs = &crs[1..]; // Skip ',' or '\n'
            if c == b'\n' {
                break;
            }
        }

        if is_correctly_ordered(&pages, unsafe{&RULES}) {
            correct_updates.push(pages);
        } else {
            wrong_updates.push(pages);
        }
    }

    (unsafe{&RULES}, correct_updates, wrong_updates)
}

pub fn p1(correct_updates: &[Update]) -> usize {
    correct_updates
        .iter()
        .map(|pages| {
            pages[pages.len()/2] as usize
        })
        .sum()
}

pub fn p2(rules: &Rules, wrong_updates: Vec<Update>) -> usize {
    wrong_updates
        .into_iter()
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

fn is_correctly_ordered(pages: &[u8], rules: &Rules) -> bool {
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
