use aoc_2023_8::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("{{crate_name}}_task_0", |b| b.iter(|| task_0(&input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
