use criterion::{criterion_group, criterion_main, Criterion};
use std::arch::asm;

fn bench(c: &mut Criterion) {
    let not_connected = vec![vec![3, 4, 6], vec![2, 4, 7], vec![2, 3, 6], vec![2, 3, 9]];

    c.bench_function("aoc_2023_20_div_func", |b| {
        b.iter(|| {
            let n = 1000;
            let mut num_h_to_cycle_conj = 1000;

            for not_connected in &not_connected {
                for &bit_idx in not_connected {
                    num_h_to_cycle_conj -= round_integer_division(n, 2 << (bit_idx - 1));
                }
            }

            assert!(num_h_to_cycle_conj > 0);
        })
    });

    c.bench_function("aoc_2023_20_div_shift", |b| {
        b.iter(|| {
            let n = 1000;
            let mut num_h_to_cycle_conj = 1000;

            for not_connected in &not_connected {
                for &bit_idx in not_connected {
                    num_h_to_cycle_conj -= (n >> bit_idx) + (n & 1 << (bit_idx - 1) != 0) as usize;
                }
            }

            assert!(num_h_to_cycle_conj > 0);
        })
    });

    c.bench_function("aoc_2023_20_div_asm", |b| {
        b.iter(|| {
            // Calculates: `num_h_to_cycle_conj -= round(N / 2^bit_idx)`
            // Divide by shifting right and then round by adding the carry bit.
            // Carry=1 indicates remainder >= 0.5, in which case we need to round up.
            let n = 1000;
            let mut num_h_to_cycle_conj = 1000;

            for not_connected in &not_connected {
                for &bit_idx in not_connected {
                    unsafe {
                        asm!(
                            "shr {n:e}, cl",
                            "sbb {num_h_to_cycle_conj:e}, {n:e}",
                            n = in(reg) n,
                            in("ecx") bit_idx,
                            num_h_to_cycle_conj = inout(reg) num_h_to_cycle_conj,
                        );
                    }
                }
            }

            assert!(num_h_to_cycle_conj > 0);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);

const fn round_integer_division(dividend: usize, divisor: usize) -> usize {
    (dividend + (divisor / 2)) / divisor
}
