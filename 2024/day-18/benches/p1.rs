use aoc_2024_18::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    parse(&aoc::read_input_to_string(), unsafe { &mut GRID });
    c.bench_function("aoc_2024_18_p1", |b| b.iter(|| p1(unsafe{&GRID})));
}

criterion_group!(benches, bench);
criterion_main!(benches);
