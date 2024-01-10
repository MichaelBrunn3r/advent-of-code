use aoc_2023_19::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (workflows, rules, _) = parse(&input);

    c.bench_function("aoc_2023_19_part_2", |b| {
        b.iter(|| black_box(part_2(rules, workflows)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
