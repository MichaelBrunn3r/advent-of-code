# 2023 Day 20

- [Solution by maneatingape](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day20.rs) (Rust)

## Benchmarks (i5-1240P, with parsing, no I/O)

- Part 1: `123.06 ns`
- Part 2: `113.77 ns`
- Parsing: `94.634 ns`

## Explanation

![Input Graph](./docs/input.svg)

### Part 1

#### Pulses to/from Broadcaster

The Broadcaster receives $n=1000$ low pulses and sends $n$ low pulses to each first FlipFlop (FF) in each cycle.\
$\rightarrow N_{Low,Broadcaster} = n + 4n = 5000$

#### Pulses between Counter FFs

Each of the 4 cycles in the graph consist out of 12 FlipFlops and 1 conjunction. They form binary counters with 12 bits:

```mermaid
flowchart LR
    vl((vl)):::flipflop --> lv((lv)):::flipflop --> rd((rd)):::flipflop --> lk((lk)):::flipflop --> hc((hc)):::flipflop --> kb((kb)):::flipflop --> pc((pc)):::flipflop --> rz((rz)):::flipflop --> fr((fr)):::flipflop --> mr((mr)):::flipflop --> jd((jd)):::flipflop --> mf((mf)):::flipflop .-> zp((zp)):::conjunction .-> vl

    classDef flipflop fill:white,stroke:black,color:black;
    classDef conjunction fill:black,stroke:white,color:white;
```

= 0000_0000_0000 $\rightarrow$ Max cycle period of $2^{12} = 4096$

| $n$ | Counter |    Pulses | $N_{Low}$     | $N_{High}$ |
| --- | ------- | --------: | ------------- | ---------- |
| 1   | 001     |     H←(L) | $0=n-ones(n)$ | $1=n$      |
| 2   | 010     |   H←L←(L) | $1=n-ones(n)$ | $2=n$      |
| 3   | 011     |     H←(L) | $1=n-ones(n)$ | $3=n$      |
| 4   | 100     | H←L←L←(L) | $3=n-ones(n)$ | $4=n$      |
| 5   | 101     |     H←(L) | $3=n-ones(n)$ | $5=n$      |
| 6   | 110     |   H←L←(L) | $4=n-ones(n)$ | $6=n$      |
| 7   | 111     |     H←(L) | $4=n-ones(n)$ | $7=n$      |

(Low pulses from the broadcaster are labeled as (L) and are not counted)

$\rightarrow N_{Low,Counter} = 4 \cdot (n - ones(n)) = 3976$\
$\rightarrow N_{High,Counter} = 4n = 4000$

#### Pulses to cycle conjunctions

$n = 1000 =$ 11_1110_1000 $\rightarrow$ Counting up to 1000 will only affect the states of 10 FFs.

The number of low/high pulses a FF sends to the conjunction depends on its position in the cycle:

| i-th FF | 1                             | 2   | 3   | 4   | 5   | 6   | 7   | ... |
| ------- | ----------------------------- | --- | --- | --- | --- | --- | --- | --- |
| L       | $\lfloor n/2^i \rfloor = 500$ | 250 | 125 | 62  | 31  | 15  | 7   |
| H       | $round(n/2^i) = 500$          | 250 | 125 | 63  | 31  | 16  | 8   |

$\rightarrow N_{Low,toCC} = \sum_{i=1..10} \lfloor n/2^i \rfloor = 994$\
$\rightarrow N_{High,toCC} = \sum_{i=1..10} round(n/2^i) = 1000$

**BUT:** In each cycle, there are 3 FFs that are not connected to the conjunction

- `zp`:
  - $N_{Low} = \sum_{i=1..10\wedge i \not\in [3,4,6]} \lfloor n/2^i \rfloor = 994 - 202 = 792$
  - $N_{High} = \sum_{i=1..10\wedge i \not\in [3,4,6]} round(n/2^i) = 1000 - 204 = 796$
  - $N_{toCC,zp} = 792 + 796 = 1588$
- `pp`:
  - $N_{Low} = 994 - 319 = 675$
  - $N_{High} = 1000 - 321 = 679$
  - $N_{toCC,pp} = 675 + 679 = 1354$
- `sj`:
  - $N_{Low} = 994 - 390 = 604$
  - $N_{High} = 1000 - 391 = 609$
  - $N_{toCC,sj} = 604 + 609 = 1213$
- `rg`:
  - $N_{Low} = 994 - 376 = 618$
  - $N_{High} = 1000 - 377 = 623$
  - $N_{toCC,rg} = 618 + 623 = 1241$

$N_{Low,toCC} = 792 + 675 + 604 + 618 = 2689$\
$N_{High,toCC} = 796 + 679 + 609 + 623 = 2707$\

#### Pulses from cycle conjunctions

A conjuntion only sends a low pulse if the most recent pulses of all connected FFs were high. Because $n=1000$ is smaller than all cycle periods, each cycle conjuction will only send high pulses.

Each cycle conjunction has 4 FFs as outputs. But because it onlys sends high pulses, the FFs will not create any pulses themselves.\
Additionaly, all cycle conjunctions are connected to `rx` via a chain of conjunctions:

```mermaid
flowchart LR
    zp((zp)):::conjunction --> xl{xl}:::inverter --> df((df)):::conjunction --> rx((rx)):::flipflop
    classDef inverter fill:black,stroke:white,color:white;
    classDef flipflop fill:white,stroke:black,color:black;
    classDef conjunction fill:black,stroke:white,color:white;
```

The directly connected conjunction (here `xl`) acts like an inverter and `df` will only send high pulses to `rx` (same reason as above):

- `zp`:
  - $N_{Low} = N_{toCC,zp} = 1588$
  - $N_{High} = (4+2)\cdot N_{toCC,zp} = 9528$
- `pp`:
  - $N_{Low} = N_{toCC,pp} = 1354$
  - $N_{High} = 6\cdot N_{toCC,pp} = 8124$
- `sj`:
  - $N_{Low} = N_{toCC,sj} = 1213$
  - $N_{High} = 6\cdot N_{toCC,sj} = 7278$
- `rg`:
  - $N_{Low} = N_{toCC,rg} = 1241$
  - $N_{High} = 6\cdot N_{toCC,rg} = 7446$

$N_{Low,fromCC} = 1588 + 1354 + 1213 + 1241 = 5396$\
$N_{High,fromCC} = 9528 + 8124 + 7278 + 7446 = 32376$

#### Total

$N_{Low} = N_{Low,Broadcaster} + N_{Low, Counter} + N_{Low,toCC} + N_{Low,fromCC} = 17061$\
$N_{High} = N_{High, Counter} + N_{High,toCC} + N_{Low,fromCC} = 39083$

### Part 2

`rx` only receives a low pulse when the conjuntions of all cycles send a low pulse in the same button press. Cycle conjunctions only send a low pulse when the cycle finished a period\
$\rightarrow$ We need to find the LCM of all cycle periods

Each cycle has a shorter cycle period than $2^{12}$, because some FFs are not connected to the conjunction\
$\rightarrow$ We sum up the power of 2 each not-connected FF represents and subtract that from $2^{12}$

Example: Cycle `zp`\
 Sum of not-connected: $2^0 + 2^2 + 2^3 + 2^5 = 1 + 4 + 8 + 32 = 45$\
 $\rightarrow$ Cycle period $= 2^{12} - 45 = 4051$

All cycle periods turn out to be co-prime $\rightarrow$ We can use the product of all cycle periods instead of LCM
