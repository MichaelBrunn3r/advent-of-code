set shell := ["nu", "-c"]

today := `date now | date to-table | get 0 | get year day | str join -`

# Run

alias r := run
run:
    cargo run -p aoc-{{today}}

alias rn := run-nightly
run-nightly:
    cargo +nightly run -p aoc-{{today}}

alias rd := run-day
run-day year day:
    cargo run -p aoc-{{year}}-{{day}}

alias rdn := run-day-nightly
run-day-nightly year day:
    cargo +nightly run -p aoc-{{year}}-{{day}}

# Test

alias t := test
test:
    cargo test -p aoc-{{today}}

alias td := test-day
test-day year day:
    cargo test -p aoc-{{year}}-{{day}}

alias tdn := test-day-nightly
test-day-nightly year day:
    cargo +nightly test -p aoc-{{year}}-{{day}}

# Init

alias i := init
init:
    nu aoc.nu init

alias id := init-day
init-day year day:
    nu aoc.nu init {{year}} {{day}}

# Update

alias u := update
update:
    nu aoc.nu update

alias ud := update-day
update-day year day:
    nu aoc.nu update {{day}} {{year}}

# Bench

alias b := bench
bench:
    cargo bench -p aoc-{{today}}

alias bn := bench-nightly
bench-nightly:
    cargo +nightly bench -p aoc-{{today}}

alias b1 := bench1
bench1:
    cargo bench -p aoc-{{today}} --bench part_1 -- --noplot

alias b1n := bench1-nightly
bench1-nightly:
    cargo +nightly bench -p aoc-{{today}} --bench part_1 -- --noplot

alias b2 := bench2
bench2:
    cargo bench -p aoc-{{today}} --bench part_2

alias bd := bench-day
bench-day year day benchmark:
    cargo bench -p aoc-{{year}}-{{day}} --bench {{benchmark}}

alias bdni := bench-day-nightly
bench-day-nightly year day benchmark:
    cargo +nightly bench -p aoc-{{year}}-{{day}} --bench {{benchmark}}

alias bdnina := bench-day-nightly-native
bench-day-nightly-native year day benchmark:
    RUSTFLAGS="-C target-cpu=native" cargo +nightly bench -p aoc-{{year}}-{{day}} --bench {{benchmark}}
