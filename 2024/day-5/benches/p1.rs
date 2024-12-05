use aoc_2024_5::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (rules, updates) = parse(&input);

    c.bench_function("aoc_2024_5_p1", |b| b.iter(|| p1(&rules, &updates)));
}

criterion_group!(benches, bench);
criterion_main!(benches);