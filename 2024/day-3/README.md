# [2024 Day 3 - Mull It Over](https://adventofcode.com/2024/day/3)

To optimize for speed I use a lot of assumptions about **my** input while still trying to make the code work for **any** input.

## Benchmarks

<!-- BEGIN benches -->
| Benchmark              | Time     |
| ---------------------- | -------- |
| [p1](./src/lib.rs#L6)  | 8.80 µs |
| [p2](./src/lib.rs#L51) | 7.28 µs |
<!-- END benches -->
<!-- BEGIN other_benches -->
| Other                         | Time       |
| ----------------------------- | ---------- |
| [p1_regex](./src/lib.rs#L117) | 132.16 µs |
| [p2_regex](./src/lib.rs#L131) | 252.24 µs |
<!-- END other_benches -->