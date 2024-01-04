use aoc_2023_20::parse::{ModuleParser, PARSER};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_20_parse", |b| {
        let parser = unsafe { &mut PARSER };
        b.iter(|| {
            parser.parse(input.as_bytes());
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
