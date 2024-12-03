$env.AOC_COOKIE = (open $"($env.FILE_PWD)/AOC_COOKIE")
let today = (date now | date to-table | get 0)

def main [] {}

def "main render_template" [year?:int, day?:int] {
    let year = $year | default ($today | get year)
    let day = $day | default ($today | get day)

    let out_dir = get_path_of_day $year $day
    mkdir -v $out_dir

    render_template $out_dir $year $day
}

def "main save_input" [year?:int, day?:int] {
    let year = $year | default ($today | get year)
    let day = $day | default ($today | get day)

    let out_dir = get_path_of_day $year $day
    mkdir -v $out_dir

    let input = fetch_input $year $day
    $input | save ($"($out_dir)/input.txt") -f
}

def get_path_of_day [year:int, day:int] {
    return $"($env.FILE_PWD)/($year)/day-($day)"
}

def render_template [dest: string, year: int, day: int] {
    cp -r ./template/* $dest

    replace_template_strings ($"($dest)/Cargo.toml") $day $year
    replace_template_strings ($"($dest)/README.md") $day $year
    replace_template_strings ($"($dest)/benches/part_1.rs") $day $year
    replace_template_strings ($"($dest)/benches/part_2.rs") $day $year
    replace_template_strings ($"($dest)/src/main.rs") $day $year
}

def replace_template_strings [file: string, year: int, day: int] {
    let content = open $file --raw
    let content = ($content | str replace --all '{{package_name}}' $'aoc-($year)-($day)')
    let content = ($content | str replace --all '{{crate_name}}' $'aoc_($year)_($day)')
    let content = ($content | str replace --all '{{year}}' $'($year)')
    let content = ($content | str replace --all '{{day}}' $'($day)')
    $content | save $file -f
}

def fetch_input [year: int, day: int] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)/input
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
#     replace_template_strings ($"($out_dir)/benches/part_1.rs") $day $year
#     replace_template_strings ($"($out_dir)/benches/part_2.rs") $day $year
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


# def fetch_day [day: int, year: int] {
#     http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day)
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
