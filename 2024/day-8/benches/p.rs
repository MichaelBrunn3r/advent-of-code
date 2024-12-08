use aoc_2024_8::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let mut node_locations = unsafe{std::mem::zeroed()};
    parse(&aoc::read_input_to_string(), &mut node_locations);
    c.bench_function("aoc_2024_8_p", |b| b.iter(|| p(&node_locations)));
}

criterion_group!(benches, bench);
criterion_main!(benches);