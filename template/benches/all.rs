use {{crate_name}}::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("{{crate_name}}_part_2", |b| b.iter(|| {
        parse(&input)
        part_1(&input)
        part_2(&input)
    }));
}

criterion_group!(benches, bench);
criterion_main!(benches);