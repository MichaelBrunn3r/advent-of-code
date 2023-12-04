use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROJECT_DIR: PathBuf =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
}

pub fn read_input_to_string() -> String {
    std::fs::read_to_string(PROJECT_DIR.join("input")).unwrap()
}

pub fn read_example_to_string(n: usize) -> String {
    std::fs::read_to_string(PROJECT_DIR.join(format!("example_{}", n))).unwrap()
}

pub fn read_solution_to_string(n: usize) -> String {
    std::fs::read_to_string(PROJECT_DIR.join(format!("solution_{}", n))).unwrap()
}

pub fn example_exists(n: usize) -> bool {
    PROJECT_DIR.join(format!("example_{}", n)).exists()
}

pub fn solution_exists(n: usize) -> bool {
    PROJECT_DIR.join(format!("solution_{}", n)).exists()
}
