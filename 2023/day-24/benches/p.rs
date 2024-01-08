use aoc_2023_24::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    c.bench_function("aoc_2023_24_parse", |b| b.iter(|| parse(&input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
