use aoc_2024_13::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let mut machines: Machines = unsafe{std::mem::zeroed()};

    // NOTE: machines does not have to be recreated every iteration, because it is completely overwritten anyways
    c.bench_function("aoc_2024_13_parse", |b| b.iter(|| parse(&input, &mut machines)));
}

criterion_group!(benches, bench);
criterion_main!(benches);