use aoc_2023_5::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (seeds, map_sections) = parse(&input);

    c.bench_function("aoc_2023_5_part_2", |b| {
        b.iter(|| part_2(&seeds, &map_sections))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
