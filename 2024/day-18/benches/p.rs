use aoc_2024_18::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench(c: &mut Criterion) {
    let bytes = parse(&aoc::read_input_to_string(), unsafe { &mut GRID });

    c.bench_function("aoc_2024_18_p", |b| {
        b.iter_batched(
            || unsafe { GRID.clone() },
            |mut grid| p(&bytes, &mut grid),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
