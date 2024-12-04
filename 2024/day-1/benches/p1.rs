use aoc_2024_1::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string(); 
    let mut left: [u32; 1000] = unsafe { std::mem::zeroed() };
    let mut right: [u32; 1000] = unsafe { std::mem::zeroed() };
    parse(&input, &mut left, &mut right);

    c.bench_function("aoc_2024_1_p1", |b| {
        b.iter_with_large_drop(|| p1(&mut left.clone(), &mut right.clone()));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
