use aoc_2023_24::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let hailstones = parse(&input);
    c.bench_function("aoc_2023_24_part_1", |b| b.iter(|| part_1(hailstones)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
