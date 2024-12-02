use aoc_2024_1::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let mut left: [u32; 1000] = unsafe { std::mem::zeroed() };
    let mut right: [u32; 1000] = unsafe { std::mem::zeroed() };

    c.bench_function("aoc_2024_1_all", |b| b.iter(|| {
        parse(&input, &mut left, &mut right);
        part_1(&mut left, &mut right);
        part_2(&left, &right);
    }));
}

criterion_group!(benches, bench);
criterion_main!(benches);
