use aoc_2024_13::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2024_13_p1", |b| b.iter(|| p1(&input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);