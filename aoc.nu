$env.AOC_COOKIE = (open AOC_COOKIE)
let date = (date now | date to-table | get 0)
let day = ($date | get day)
let year = ($date | get year)

def main [] {}

def "main load_cookie" [] {
    export-env {
        $env.AOC_COOKIE = (open AOC_COOKIE)
    }
}

def "main input" [] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)/input
}

def "main desc" [n: int] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/2023/day/3 | query web --query '.day-desc' | get $n
}