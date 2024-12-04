use aoc_2023_8::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
    let (instructions, network, nodes_ending_in_a) = parse(&input);

    c.bench_function("aoc_2023_8_p2", |b| {
        b.iter(|| p2(instructions, network, &nodes_ending_in_a))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
