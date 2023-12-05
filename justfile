set shell := ["nu", "-c"]
alias r := run
alias rd := run-day
alias i := init
alias u := update
alias t := test
alias td := test-day

today := `date now | date to-table | get 0 | get year day | str join -`

run:
    cargo run -p aoc-{{today}}

run-day year day:
    cargo run -p aoc-{{year}}-{{day}}

test:
    cargo test -p aoc-{{today}}

test-day year day:
    cargo test -p aoc-{{year}}-{{day}}

init:
    nu aoc.nu init

update:
    nu aoc.nu update