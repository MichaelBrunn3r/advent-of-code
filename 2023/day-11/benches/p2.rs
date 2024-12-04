use aoc_2023_11::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_11_p2", |b| b.iter(|| p2(&input, 999_999)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
