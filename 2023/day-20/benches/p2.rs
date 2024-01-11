use aoc_2023_20::{parse::parse, *};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (broadcaster_outputs, cycle_conjunctions, modules) = parse(&input);

    c.bench_function("aoc_2023_20_part_2", |b| {
        b.iter(|| black_box(part_2(&broadcaster_outputs, modules, &cycle_conjunctions)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
