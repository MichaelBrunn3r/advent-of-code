use aoc_2023_11::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_11_part_1", |b| {
        b.iter(|| black_box(part_1(&input)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
