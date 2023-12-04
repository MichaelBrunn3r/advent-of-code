$env.AOC_COOKIE = (open $"($env.FILE_PWD)/AOC_COOKIE")
let today = (date now | date to-table | get 0)

def main [] {}

def "main in" [day:int = -1, year:int = -1] {
    let day = if ($day == -1) {$today | get day} else $day
    let year = if ($year == -1) {$today | get year} else $year
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)/input
}

def "main desc" [n: int, day:int = -1, year:int = -1] {
    let day = if ($day == -1) {$today | get day} else $day
    let year = if ($year == -1) {$today | get year} else $year
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day) | query web --query '.day-desc' | get $n
}

def "main ex" [n: int, day:int = -1, year:int = -1] -> Table {
    let day = if ($day == -1) {$today | get day} else $day
    let year = if ($year == -1) {$today | get year} else $year

    let result = http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)
    let example = $result | query web --query 'pre' | get $n
    let solution = $result | query web --query 'code em' | get $n

    let out_dir = $"($env.FILE_PWD)\\($year)\\($day)"
    mkdir -v $out_dir

    $example | save ($"($out_dir)\\example_($n)") -f
    $solution | save ($"($out_dir)\\solution_($n)") -f
}