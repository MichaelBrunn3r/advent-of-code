# [2023 Day 7: Camel Cards](https://adventofcode.com/2023/day/7)

- [Solution from orlp](https://github.com/orlp/aoc2023/blob/master/src/bin/day07.rs) [Rust]

## Benchmarks (with parsing, no I/O)

| CPU                  | Part 1      | Part 2      |
| -------------------- | ----------- | ----------- |
| i5-1240P@1.7-4.4GHz  | `21.549 µs` | `21.490 µs` |
| i5-12600K@3.7-4.9GHz | `19.384 µs` | `19.353 µs` |

## Explanation

### Counting pairs, triples, etc.

- `is_joker`: Indicates which positions are jokers. 1 nibble (4 bits) per position, so 5
- `counts`: Indicates how many times each card appears. 1 nibble per card label
- `count_occurences`: Indicates how many times each count occurs, i.e. how many cards occur 0,1,2,3,4,5 times. Sum of nibbles is 13 (number of different labels)

```
cards:                      KTJJT
is_joker:                 0x00110

                    AKQJT98765432
counts:           0x0102200000000

                           543210
count occurences: 0x000000000021a
```

```
cards:                      QQQJA
is_joker:                 0x00010

                    AKQJT98765432
counts:           0x1031000000000

                           543210
count occurences: 0x000000000102a
```

### Determine if hand is FiveOfAKind, FourOfAKind, etc.

`count_occurences` can only be one of these values:

```
  543210
0x10000c -> FiveOfAKind
0x01001b -> FourOfAKind
0x00110b -> FullHouse
0x00102a -> ThreeOfAKind
0x00021a -> TwoPairs
0x000139 -> OnePair
0x000058 -> FiveUnique
```
