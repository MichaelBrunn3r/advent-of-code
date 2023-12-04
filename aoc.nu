$env.AOC_COOKIE = (open $"($env.FILE_PWD)/AOC_COOKIE")
let today = (date now | date to-table | get 0)

def main [] {}

def "main init" [day?:int, year?:int] {
    let day = if ($day == null) {$today | get day} else $day
    let year = if ($year == null) {$today | get year} else $year

    # Create day directory
    let out_dir = $"($env.FILE_PWD)\\($year)\\($day)"
    mkdir -v $out_dir

    # Create Description.md
    let html_day = fetch_day $day $year
    create_description $html_day ($"($out_dir)\\Description.md")

    # Copy template
    copy_template $out_dir

    # Save examples and solutions
    create_examples_with_solutions $html_day $out_dir

    create_input $day $year $out_dir
}

def create_input [day: int, year: int, out_dir: string] {
    let input = fetch_input $day $year
    $input | save ($"($out_dir)\\input") -f
}

def create_description [html: string, dest: string] {
    html_to_markdown $html | query web --query '.day-desc' | save $dest -f
}

def copy_template [dest: string] {
    cp -r template\* $dest
}

def create_examples_with_solutions [html: string, dir: string] {
    let examples = $html | query web --query 'pre'
    let solutions = $html | query web --query 'code em'
    for $i in 0..(($examples | length) - 1) {
        let example = $examples | get $i
        let solution = $solutions | get $i

        $example | save ($"($dir)\\example_($i)") -f
        $solution | save ($"($dir)\\solution_($i)") -f
    }
}


def fetch_day [day: int, year: int] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)
}

def fetch_input [day: int, year: int] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)/input
}

def html_to_markdown [html: string] {
    mut html = $html
    $html = ($html | str replace --all -r '<code>([\d \*=]+)</code>' '`$1`')
    $html = ($html | str replace --all -r '<code><em>(\d+)</em></code>' '**`$1`**')
    $html = ($html | str replace --all -r '<em>([^<]+)</em>' '**$1**')
    $html = ($html | str replace -r '<h2>--- (.+) ---</h2>' '# $1
')
    $html = ($html | str replace -r '<h2 id="part2">--- (.+) ---</h2>' '## $1
')
    $html = ($html | str replace --all '<pre><code>' '```' | str replace --all '</code></pre>' '```')
    $html = ($html | str replace --all -r '<p>(.*)</p>' '$1
')
    $html
}