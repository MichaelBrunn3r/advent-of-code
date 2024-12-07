#! /run/current-system/sw/bin/nu

$env.AOC_COOKIE = open $"($env.FILE_PWD)/AOC_COOKIE"
let today = (date now | date to-table | get 0)

#
# CLI commands
# 

def main [] {}

def "main render_template" [year?:int, day?:int] {
    let year = $year | default ($today | get year)
    let day = $day | default ($today | get day)

    let html_day = fetch_day $year $day
    let $title = $html_day | parse -r '-- Day \d+: (?<title>.*) ---' | first | get title

    let out_dir = path_day_root $year $day
    mkdir -v $out_dir

    render_template $out_dir $year $day $title
}

def "main save_input" [year?:int, day?:int] {
    let year = $year | default ($today | get year)
    let day = $day | default ($today | get day)

    let out_dir = path_day_root $year $day
    mkdir -v $out_dir

    let input = fetch_input $year $day
    $input | save ($"($out_dir)/input.txt") -f
}

def "main gen_benchmarks_table" [year?:int, day?:int] {
    let year = $year | default ($today | get year)
    let day = $day | default ($today | get day)
    let day_root = path_day_root $year $day

    let text_lib = open $"($day_root)/src/lib.rs"

    let bench_name = ls $"($day_root)/benches" | each {|e| $e.name | path basename | str replace ".rs" ""}
    let bench_loc = $bench_name | each {|n| $text_lib | get_line_of_match $"pub fn ($n)" }
    let bench_name_md = $bench_name | zip $bench_loc | each {|e| if $e.1 != -1 {$"[($e.0)]\(./src/lib.rs#L($e.1)\)"} else {$e.0}}
    let bench_time = $bench_name | each {|n| get_bench_time_ns $year $day $n | format_bench_time }
    let all_benches = $bench_name | wrap name | merge ($bench_time | wrap time) | merge ($bench_name_md | wrap name_md) | merge ($bench_loc | wrap loc) | sort-by loc

    let benches = $all_benches | filter {|row| $row.name in [parse p1 p2]} | select name_md time | rename Benchmark Time
    let other_benches = $all_benches | filter {|row| not ($row.name in [parse p1 p2])} | select name_md time | rename Other Time

    mut text_readme = open $"($day_root)/README.md"
    $text_readme = str_replace_region $text_readme benches $"\n($benches | to md -p)\n"
    $text_readme = str_replace_region $text_readme other_benches $"\n($other_benches | to md -p)\n"
    $text_readme | save -f $"($day_root)/README.md"
}

#
# Utils
#

def get_line_of_match [pattern: string] {
    let matches = $in | enumerate_lines | update str {|row| $row.str | parse --regex $pattern } | filter {|row| $row.str | is-not-empty }
    if ($matches | is-empty) {
        -1
    } else {
        $matches | first | get line
    }
}

def enumerate_lines [] {
    $in | lines | enumerate | rename line str | update line {|row| $row.line + 1}
}

def get_bench_time_ns [year: int, day:int, benchmark: string] {
    open $"target/criterion/aoc_($year)_($day)_($benchmark)/new/estimates.json" | get mean.point_estimate
}

def format_bench_time [] {
    if $in <= 1000 {
        $"($in)ns" | into duration | format duration ns
    } else if $in <= 1000000 {
        $"($in)ns" | into duration | format duration us
    } else if $in <= 1000000000 {
        $"($in)ns" | into duration | format duration ms
    }
}

def str_replace_region [str: string, name: string, replace: string] {
    let idx_begin = ($str | str index-of $"<!-- BEGIN ($name) -->")
    let idx_end = ($str | str index-of $"<!-- END ($name) -->")

    ($str | str substring ..($idx_begin + 14 + ($name | str length))) + $replace + ($str | str substring $idx_end..)
}

def path_day_root [year:int, day:int] {
    return $"($env.FILE_PWD)/../($year)/day-($day)"
}

def render_template [dest: string, year: int, day: int, title: string] {
    cp -r ./template/* $dest

    replace_template_strings ($"($dest)/Cargo.toml") $year $day $title
    replace_template_strings ($"($dest)/README.md") $year $day $title
    replace_template_strings ($"($dest)/benches/p1.rs") $year $day $title
    replace_template_strings ($"($dest)/benches/p2.rs") $year $day $title
    replace_template_strings ($"($dest)/src/main.rs") $year $day $title
}

def replace_template_strings [file: string, year: int, day: int, title: string] {
    let content = open $file --raw
    let content = ($content | str replace --all '{{package_name}}' $'aoc-($year)-($day)')
    let content = ($content | str replace --all '{{crate_name}}' $'aoc_($year)_($day)')
    let content = ($content | str replace --all '{{year}}' $'($year)')
    let content = ($content | str replace --all '{{day}}' $'($day)')
    let content = ($content | str replace --all '{{title}}' $'($title)')
    $content | save $file -f
}

def fetch_input [year: int, day: int] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)/input
}

def fetch_day [year: int, day: int] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)
}

# def "main init" [day?:int, year?:int] {
#     let day = if ($day == null) {$today | get day} else $day
#     let year = if ($year == null) {$today | get year} else $year

#     # Create day directory
#     let out_dir = $"($env.FILE_PWD)/($year)/day-($day)"
#     mkdir -v $out_dir

#     # Create Description.md
#     let html_day = fetch_day $day $year
#     # create_description $html_day ($"($out_dir)/TASKS.md") $day $year

#     # Copy template
#     copy_template $out_dir

#     # Save examples and solutions
#     create_examples_with_solutions $html_day $out_dir

#     create_input $day $year $out_dir

#     replace_template_strings ($"($out_dir)/Cargo.toml") $day $year
#     replace_template_strings ($"($out_dir)/README.md") $day $year
#     replace_template_strings ($"($out_dir)/benches/p1.rs") $day $year
#     replace_template_strings ($"($out_dir)/benches/p2.rs") $day $year
#     replace_template_strings ($"($out_dir)/src/main.rs") $day $year
# }

# def "main update" [day?:int, year?:int] {
#     let day = if ($day == null) {$today | get day} else $day
#     let year = if ($year == null) {$today | get year} else $year

#     # Create day directory
#     let out_dir = $"($env.FILE_PWD)/($year)/day-($day)"

#     # Create Description.md
#     let html_day = fetch_day $day $year
#     # create_description $html_day ($"($out_dir)/TASKS.md") $day $year

#     # Save examples and solutions
#     create_examples_with_solutions $html_day $out_dir
# }

# def create_input [day: int, year: int, out_dir: string] {
#     let input = fetch_input $day $year
#     $input | save ($"($out_dir)/input.txt") -f
# }

# def create_description [html: string, dest: string, day: int, year: int] {
#     day_html_to_markdown $html $day $year | query web --query '.day-desc' | save $dest -f
# }

# def create_examples_with_solutions [html: string, project_dir: string] {
#     let examples_dir = ($"($project_dir)/examples")
#     mkdir -v $examples_dir

#     let examples = $html | query web -q '.day-desc pre'
#     let solutions = $html | query web -q '.day-desc code em'
#     for $i in 0..(($examples | length) - 1) {
#         let example = $examples | get $i
#         let solution = $solutions | reverse | get 0

#         let ex_path = ($"($examples_dir)/($i).txt")
#         if ($ex_path | path exists ) {} else {
#             $example | save $ex_path
#         }

#         let sol_path = ($"($examples_dir)/($i)_solution.txt")
#         if ($sol_path | path exists ) {} else {
#             $solution | save $sol_path
#         }
#     }
# }

# def day_html_to_markdown [html: string, day: int, year: int] {
#     mut html = $html
#     $html = ($html | str replace --all -r '<code>([\d \*=]+)</code>' '`$1`')
#     $html = ($html | str replace --all -r '<code><em>(\d+)</em></code>' '**`$1`**')
#     $html = ($html | str replace --all -r '<em>([^<]+)</em>' '**$1**')
#     let link = "(" + ($"https://adventofcode.com/($year)/day/($day)") + ")"
#     $html = ($html | str replace -r '<h2>--- (.+) ---</h2>' $'# [$1]($link)
# ')
#     $html = ($html | str replace -r '<h2 id="part2">--- (.+) ---</h2>' '## $1
# ')
#     $html = ($html | str replace --all '<pre><code>' '```' | str replace --all '</code></pre>' '```')
#     $html = ($html | str replace --all -r '<p>(.*)</p>' '$1
# ')
#     $html = ($html | str replace --all -r '<a href="([^"]+)"[^>]*>([^<]*)</a>' '[$2]($1)')
#     $html
# }
