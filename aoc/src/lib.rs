use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROJECT_DIR: PathBuf =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    pub static ref EXAMPLES_DIR: PathBuf = PROJECT_DIR.join("examples");
}

pub fn read_input_to_string() -> String {
    std::fs::read_to_string(PROJECT_DIR.join("input")).unwrap()
}

pub fn read_example_to_string(n: usize) -> String {
    std::fs::read_to_string(example_path(n)).unwrap()
}

pub fn read_solution_to_string(n: usize) -> String {
    std::fs::read_to_string(solution_path(n)).unwrap()
}

pub fn solution_path(n: usize) -> PathBuf {
    EXAMPLES_DIR.join(format!("{}_solution", n))
}

pub fn example_path(n: usize) -> PathBuf {
    EXAMPLES_DIR.join(format!("{}", n))
}

pub fn example_exists(n: usize) -> bool {
    example_path(n).exists()
}

pub fn solution_exists(n: usize) -> bool {
    solution_path(n).exists()
}
