use aoc_2023_10::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let grid = parse(aoc::read_input_to_string());

    c.bench_function("aoc_2023_10_part_1", |b| b.iter(|| part_1(&grid)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
