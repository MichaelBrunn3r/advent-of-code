use aoc::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

fn bench(c: &mut Criterion) {
    let mut rng: Pcg64 = Seeder::from("num_digits").make_rng();

    let mut group_all = c.benchmark_group("all");
    let mixed = (0..100_000)
        .map(|_| rng.gen_range(0..usize::MAX))
        .collect_vec();

    group_all.bench_function("aoc_num_digits_ilog10", |b: &mut criterion::Bencher<'_>| {
        b.iter(|| mixed.iter().all(|&n| black_box(digits_ilog10(n)) > 0))
    });
    group_all.bench_function(
        "aoc_num_digits",
        |b: &mut criterion::Bencher<'_>| {
            b.iter(|| mixed.iter().all(|&n| black_box(n.num_digits()) > 0))
        },
    );
    group_all.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

fn digits_ilog10(num: usize) -> usize {
    if num == 0 {
        return 1;
    }
    (num.ilog10() as usize) + 1
}
