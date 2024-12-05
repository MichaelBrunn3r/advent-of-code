use aoc_2024_3::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2024_3_p1_regex", |b| b.iter(|| p1_regex(&input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);