use aoc_2023_14::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_14_part_2", |b| {
        let mut input = input.clone();
        b.iter(|| part_2(&mut input))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
