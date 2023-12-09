set shell := ["nu", "-c"]

today := `date now | date to-table | get 0 | get year day | str join -`

alias r := run
run:
    cargo run -p aoc-{{today}}

alias rd := run-day
run-day year day:
    cargo run -p aoc-{{year}}-{{day}}


alias t := test
test:
    cargo test -p aoc-{{today}}

alias td := test-day
test-day year day:
    cargo test -p aoc-{{year}}-{{day}}

alias i := init
init:
    nu aoc.nu init

alias id := init-day
init-day year day:
    nu aoc.nu init {{year}} {{day}}

alias u := update
update:
    nu aoc.nu update

alias b := bench
bench:
    cargo bench -p aoc-{{today}}

alias b1 := bench1
bench1:
    cargo bench -p aoc-{{today}} --bench part_1

alias b2 := bench2
bench2:
    cargo bench -p aoc-{{today}} --bench part_2

alias bd := bench-day
bench-day year day:
    cargo bench -p aoc-{{year}}-{{day}}

alias bd1 := bench1-day
bench1-day year day:
    cargo bench -p aoc-{{year}}-{{day}} --bench part_1

alias bd2 := bench2-day
bench2-day year day:
    cargo bench -p aoc-{{year}}-{{day}} --bench part_2