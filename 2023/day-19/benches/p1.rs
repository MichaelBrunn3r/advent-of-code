use aoc_2023_19::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (rules, workflows, workflow_in_id, parts, _) = parse(&input);

    c.bench_function("aoc_2023_19_part_1", |b| {
        b.iter(|| part_1(&rules, &workflows, workflow_in_id, &parts))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
