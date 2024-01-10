use aoc_2023_19::parse;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let input = aoc::read_input_to_string();

    c.bench_function("aoc_2023_19_parse", |b| b.iter(|| black_box(parse(&input))));

    // let input = input.split_once("\n\n").unwrap().1;

    // c.bench_function("aoc_2023_19_parse_parts", |b| {
    //     b.iter(|| {
    //         let mut crs: Cursor<u8> = input.as_ptr().into();
    //         black_box(parse_parts(&mut crs))
    //     })
    // });
}

criterion_group!(benches, bench);
criterion_main!(benches);
