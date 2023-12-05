set shell := ["nu", "-c"]
alias r := run
alias i := init
alias u := update
alias t := test

today := `date now | date to-table | get 0 | get year day | str join -`

run:
    cargo run -p aoc-{{today}}

test:
    cargo test -p aoc-{{today}}

init:
    nu aoc.nu init

update:
    nu aoc.nu update