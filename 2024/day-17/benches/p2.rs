use aoc_2024_17::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let (_, prog) = parse(&aoc::read_input_to_string());
    c.bench_function("aoc_2024_17_p2", |b| b.iter(|| p2(&prog)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
