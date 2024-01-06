use aoc_2023_5::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_5_part_1_parse", |b| {
        b.iter(|| unsafe {
            let mut data = input.as_ptr();
            data = data.add("seeds: ".len());

            parse_seeds(&mut data);
            parse_map_sections(&mut data);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
