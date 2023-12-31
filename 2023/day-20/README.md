# 2023 Day 20

- [Solution by maneatingape](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day20.rs) (Rust)

## Benchmarks (i5-1240P, with parsing, no I/O)

- Part 1: `170.06 µs`
- Part 2: `2.8706 µs`

## Explanation

![Input Graph](./input.svg)

- There are 4 cycles in the graph
- Each cycle consists of 12 FlipFlops and 1 conjunction
- `rx` only receives a signal when the conjuntions of all cycles send a pulse in the same button press -> We need to find the LCM of all cycle lengths
- The FlipFlops in a cycle can be thought of as binary numbers with 12 bits -> Max cycle length of `2^12 = 4096`
- Each cycle has a shorter cycle length than `2^12`, because some FlipFlops are not connected to the conjunction -> We sum up the bits each not-connected FlipFlop represents and subtract that from `2^12`

**Example:**

- Cycle `zp`: Sum of not-connected: `1 + 4 + 8 + 32 = 45` -> Cycle length: `2^12 - 45 = 4051`
