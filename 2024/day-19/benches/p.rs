use aoc_2024_19::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();
        let (patterns, designs) = parse(&input);

    c.bench_function("aoc_2024_19_p", |b| b.iter(|| p(&patterns, designs)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
