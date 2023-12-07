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

alias b0 := bench0
bench0:
    cargo bench -p aoc-{{today}} --bench task_0

alias b1 := bench1
bench1:
    cargo bench -p aoc-{{today}} --bench task_1

alias bd := bench-day
bench-day year day:
    cargo bench -p aoc-{{year}}-{{day}}