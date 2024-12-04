use aoc_2023_17::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_17_p1", |b| {
        b.iter(|| p1(&mut input.clone()))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
