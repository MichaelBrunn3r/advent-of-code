use aoc_2024_10::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2024_10_p", |b| b.iter(|| p(& input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);