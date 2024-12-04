use aoc_2023_2::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_2_p1", |b| {
        b.iter(|| black_box(p1(&input)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
