use aoc_2024_5::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (rules, _, wrong_updates) = parse(&input);

    c.bench_function(
        "aoc_2024_5_p2", 
        |b| b.iter_batched(
            || {wrong_updates.clone()},
            |wrong_updates| p2(&rules, wrong_updates),
            BatchSize::SmallInput));
}

criterion_group!(benches, bench);
criterion_main!(benches);