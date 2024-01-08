use aoc_2023_19::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (rules, workflows, _, _, name_to_id) = parse(&input);

    c.bench_function("aoc_2023_19_part_2", |b| {
        b.iter(|| part_2(&rules, &workflows, &name_to_id))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
