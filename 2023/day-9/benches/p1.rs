use aoc_2023_9::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let data = parse(&input);

    c.bench_function("aoc_2023_9_part_1", |b| b.iter(|| part_1(data)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
