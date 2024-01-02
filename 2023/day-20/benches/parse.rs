use aoc_2023_20::parse::ModuleParser;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_20_parse", |b| {
        b.iter(|| {
            let mut parser = ModuleParser::new(input.as_bytes());
            parser.parse();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
