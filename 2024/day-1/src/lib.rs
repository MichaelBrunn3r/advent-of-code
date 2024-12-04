use aoc::U8SliceExt;

pub fn parse(input: &str, left: &mut [u32], right: &mut [u32]) {
    for (i, x) in input.as_bytes().chunks_exact(14).enumerate() {
        left[i] = x.parse_n_ascii_digits(5);
        right[i] = x[8..].parse_n_ascii_digits(5);
    }
}

pub fn p1(left: &mut [u32], right: &mut [u32]) -> usize {
    debug_assert!(!left.is_sorted());
    debug_assert!(!right.is_sorted());

    left.sort_unstable();
    right.sort_unstable();

    left
        .iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(*r) as usize)
        .sum()
}

pub fn p2(left: &[u32], right: &[u32]) -> usize {
    debug_assert!(left.is_sorted());
    debug_assert!(right.is_sorted());

    let mut sum = 0usize;   
    let mut ri = 0usize;
    for l in left {
        if ri < right.len() && *l < right[ri] {
            continue;
        }

        while ri < right.len() && *l > right[ri] {
            ri += 1;
        }

        while ri < right.len() && *l == right[ri] {
            sum += *l as usize;
            ri += 1;
        }
    }

    sum
}
