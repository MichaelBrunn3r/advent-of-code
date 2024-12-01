use aoc_2023_15::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_15_part_2", |b| b.iter(|| part_2(&input)));
    // c.bench_function("aoc_2023_15_part_2_avx", |b| b.iter(|| part_2_avx(&input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
