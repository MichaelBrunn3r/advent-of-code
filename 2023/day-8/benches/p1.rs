use aoc_2023_8::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (instructions, network, _) = parse(&input);

    c.bench_function("aoc_2023_8_p1", |b| {
        b.iter(|| p1(instructions, network))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
