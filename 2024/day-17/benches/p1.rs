use aoc_2024_17::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let (a, prog) = parse(&aoc::read_input_to_string());
    let mut out = [b','; PROGRAM_LEN + 1];
    c.bench_function("aoc_2024_17_p1", |b| b.iter(|| black_box({let _ = p1(a, &prog, &mut out);})));
}

criterion_group!(benches, bench);
criterion_main!(benches);
