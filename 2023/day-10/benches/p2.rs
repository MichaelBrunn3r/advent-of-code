use aoc_2023_10::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let grid = parse(aoc::read_input_to_string());

    c.bench_function("aoc_2023_10_p2", |b| {
        b.iter(|| p2(&mut grid.clone()))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
