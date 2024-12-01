use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let in_day1 = std::fs::read_to_string("./day-1/input.txt").unwrap();
    let in_day2 = std::fs::read_to_string("./day-2/input.txt").unwrap();
    let in_day3 = std::fs::read_to_string("./day-3/input.txt").unwrap();
    let in_day4 = std::fs::read_to_string("./day-4/input.txt").unwrap();
    let in_day5 = std::fs::read_to_string("./day-5/input.txt").unwrap();

    c.bench_function("aoc_2023_all", |b| {
        b.iter(|| {
            // Day 1
            aoc_2023_1::part_1(&in_day1);
            aoc_2023_1::part_2(&in_day1);

            // Day 2
            aoc_2023_2::part_1(&in_day2);
            aoc_2023_2::part_2(&in_day2);

            // Day 3
            let in_day3_parsed = aoc_2023_3::prepare_input(&in_day3);
            aoc_2023_3::part_1(&in_day3);
            aoc_2023_3::part_2(&in_day3_parsed);

            // Day 4
            aoc_2023_4::part_1(&in_day4);
            aoc_2023_4::part_2(&in_day4);

            // Day 5
            let in_day5_parsed = aoc_2023_5::parse(&in_day5);
            aoc_2023_5::part_1(&in_day5_parsed);
            aoc_2023_5::part_2(&in_day5_parsed);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
