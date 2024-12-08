use aoc_2024_8::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let node_locations = parse(&aoc::read_input_to_string());
    c.bench_function("aoc_2024_8_p", |b| b.iter(|| p(&node_locations)));
}

criterion_group!(benches, bench);
criterion_main!(benches);