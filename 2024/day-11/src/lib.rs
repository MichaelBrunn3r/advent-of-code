use aoc::prelude::*;
use fxhash::FxHashMap;

pub fn p(input: &str) -> (usize, usize) {
    let mut stones = FxHashMap::default();
    input[..input.len() - 1]
        .split(" ")
        .map(|stone| stone.as_bytes().parse_ascii_digits())
        .for_each(|stone| *stones.entry(stone).or_default() += 1);

    let stones_25 = count_stones(25, &mut stones);
    let stones_75 = count_stones(50, &mut stones);

    (stones_25, stones_75)
}

fn count_stones(blinks: u8, stones: &mut FxHashMap<usize, usize>) -> usize {
    let mut new = FxHashMap::default(); // see Note 1

    for _ in 0..blinks {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new.entry(1).or_default() += count;
            } else {
                let num_digits = stone.digits();
                if num_digits.even() {
                    let left = stone / (pow_10(num_digits / 2));
                    let right = stone % (pow_10(num_digits / 2));
        
                    *new.entry(left).or_default() += count;
                    *new.entry(right).or_default() += count;
                } else {
                    *new.entry(stone * 2024).or_default() += count;
                }
            }
        }

        std::mem::swap(stones, &mut new);
    }

    stones.values().sum()
}

// Note 1: The trick is that we deduplicate all stones.
//         In each iteration we process e.g. ALL stones with a 1, ALL stones with a 23425, ...

fn pow_10(exp: usize) -> usize {
    debug_assert!(exp <= 19, "10usize^x overflows for x > 19");
    static POW_10: [usize; 20] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
        1000000000000,
        10000000000000,
        100000000000000,
        1000000000000000,
        10000000000000000,
        100000000000000000,
        1000000000000000000,
        10000000000000000000
    ];

    POW_10[exp]
}
