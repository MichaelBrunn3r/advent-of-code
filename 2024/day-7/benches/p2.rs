use aoc_2024_7::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let lines = parse(&aoc::read_input_to_string());

    c.bench_function("aoc_2024_7_p2", |b| b.iter(|| p2(&lines)));
}

criterion_group!(benches, bench);
criterion_main!(benches);