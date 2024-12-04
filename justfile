set shell := ["nu", "-c"]

today := `date now | date to-table | get 0 | get year day | str join -`

# Init

alias id := init-day
init-day year day:
    nu scripts/aoc.nu render_template {{year}} {{day}}
    nu scripts/aoc.nu save_input {{year}} {{day}}

render_template year day:
    nu scripts/aoc.nu render_template {{year}} {{day}}

# Run

alias r := run
run:
    cargo run -p aoc-{{today}}

alias rd := run-day
run-day year day:
    cargo run -p aoc-{{year}}-{{day}}

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

# Update

alias u := update
update:
    nu scripts/aoc.nu update

alias ud := update-day
update-day year day:
    nu scripts/aoc.nu update {{day}} {{year}}

# Bench

alias b := bench
bench:
    cargo bench -p aoc-{{today}}

alias b1 := bench1
bench1:
    cargo bench -p aoc-{{today}} --bench p1 -- --noplot

alias b2 := bench2
bench2:
    cargo bench -p aoc-{{today}} --bench p2

alias bd := bench-day
bench-day year day benchmark:
    cargo bench -p aoc-{{year}}-{{day}} --bench {{benchmark}}
