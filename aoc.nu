$env.AOC_COOKIE = (open $"($env.FILE_PWD)/AOC_COOKIE")
let today = (date now | date to-table | get 0)

def main [] {}

def "main desc" [n: int, day:int = -1, year:int = -1] {
    let day = if ($day == -1) {$today | get day} else $day
    let year = if ($year == -1) {$today | get year} else $year
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day) | query web --query '.day-desc' | get $n
}

def "main init" [day:int = -1, year:int = -1] {
    let day = if ($day == -1) {$today | get day} else $day
    let year = if ($year == -1) {$today | get year} else $year

    # Copy template
    let out_dir = $"($env.FILE_PWD)\\($year)\\($day)"
    mkdir -v $out_dir
    cp -r template\* ($"($env.FILE_PWD)\\($year)\\($day)")

    let result = http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)

    # Save examples and solutions
    let examples = $result | query web --query 'pre'
    let solutions = $result | query web --query 'code em'
    for $i in 0..(($examples | length) - 1) {
        let example = $examples | get $i
        let solution = $solutions | get $i

        $example | save ($"($out_dir)\\example_($i)") -f
        $solution | save ($"($out_dir)\\solution_($i)") -f
    }

    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)/input | save ($"($out_dir)\\input") -f
}
