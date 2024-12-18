use aoc_2024_18::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    c.bench_function("aoc_2024_18_parse", |b| {
        b.iter(|| parse(&aoc::read_input_to_string(), unsafe { &mut GRID }))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
