use aoc_2023_19::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (workflows, rules, parts) = parse(&input);

    c.bench_function("aoc_2023_19_part_1", |b| {
        b.iter(|| black_box(part_1(workflows, rules, parts)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
