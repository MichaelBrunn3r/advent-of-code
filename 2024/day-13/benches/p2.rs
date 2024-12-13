use aoc_2024_13::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let mut machines: Machines = unsafe{std::mem::zeroed()};
    parse(&aoc::read_input_to_string(), &mut machines);
    c.bench_function("aoc_2024_13_p2", |b| b.iter(|| p2(&machines)));
}

criterion_group!(benches, bench);
criterion_main!(benches);