use aoc::prelude::*;
use itertools::Itertools;

const NUM_LINES_RULES: usize = 1176;
const LINE_LENGTH_RULES: usize = 6; // ASSUMPTION 1: All rules match the pattern '\d\d\|\d\d\n'
const NUM_LINES_UPDATES: usize = 191;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    let bytes = input.as_bytes();

    let mut rules = vec![Vec::<u8>::new(); 100];
    bytes
        .chunks_exact(LINE_LENGTH_RULES)
        .take(NUM_LINES_RULES)
        .for_each(|l| {
            let a = l.parse_n_ascii_digits(2) as u8;
            let b = l[3..].parse_n_ascii_digits(2) as usize;
            rules[b].push(a);
        });

    rules
}

pub fn p1(input: &str, rules: &[Vec<u8>]) -> usize {
    let bytes = input.as_bytes();

    bytes[(LINE_LENGTH_RULES * NUM_LINES_RULES)+1..]
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

pub fn p2(input: &str, rules: &[Vec<u8>]) -> usize {
    let bytes = input.as_bytes();

    bytes[(LINE_LENGTH_RULES * NUM_LINES_RULES)+1..]
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
        .filter(|pages| !is_correctly_ordered(pages, &rules))
        .map(|mut pages| {
            for i in 1..pages.len() {
                let page = pages[i];
                for il in (0..i).rev() {
                    let before_il = &rules[pages[il] as usize];
                    if !before_il.contains(&page) {
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

fn is_correctly_ordered(pages: &[u8], rules: &[Vec<u8>]) -> bool {
    for i in 0..pages.len() {
        let page = pages[i];
        for il in 0..i {
            let before_il = &rules[pages[il] as usize];
            if before_il.contains(&page) {
                return false
            }
        }

        let before_i = &rules[page as usize];
        for ir in i+1..pages.len() {
            if before_i.contains(&pages[ir]) {
                return false;
            }
        }
    }

    true
}