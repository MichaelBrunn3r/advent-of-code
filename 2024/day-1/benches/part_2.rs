use aoc_2024_1::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string(); 
    let mut left: [u32; 1000] = unsafe { std::mem::zeroed() };
    let mut right: [u32; 1000] = unsafe { std::mem::zeroed() };
    parse(&input, &mut left, &mut right);
    
    left.sort_unstable();
    right.sort_unstable();

    c.bench_function("aoc_2024_1_part_2", |b| b.iter(|| part_2(&left, &right)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
