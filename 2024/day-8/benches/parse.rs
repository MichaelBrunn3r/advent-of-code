use aoc::ConstVec;
use aoc_2024_8::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    c.bench_function("aoc_2024_8_parse", |b| b.iter_batched(
        || unsafe{std::mem::zeroed()}, 
        |mut node_locations| parse(&input , &mut node_locations),
        BatchSize::SmallInput));
}

criterion_group!(benches, bench);
criterion_main!(benches);