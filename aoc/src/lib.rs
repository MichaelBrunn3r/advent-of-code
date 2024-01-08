mod assert;
mod constvec;
mod coords;
mod iter;
mod number;
mod parsing;
mod point;
mod range;
mod slice;
mod string;
mod u8;

pub use assert::*;
pub use constvec::*;
pub use coords::*;
pub use iter::*;
pub use number::*;
pub use parsing::*;
pub use point::*;
pub use range::*;
pub use slice::*;
pub use string::*;
pub use u8::*;

use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROJECT_DIR: PathBuf =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    pub static ref EXAMPLES_DIR: PathBuf = PROJECT_DIR.join("examples");
}

pub fn read_input_to_string() -> String {
    std::fs::read_to_string(PROJECT_DIR.join("input.txt")).unwrap()
}

pub fn read_example_to_string(n: usize) -> String {
    std::fs::read_to_string(example_path(n)).unwrap()
}

pub fn read_solution_to_string(n: usize) -> String {
    std::fs::read_to_string(solution_path(n)).unwrap()
}

pub fn solution_path(n: usize) -> PathBuf {
    EXAMPLES_DIR.join(format!("{}_solution.txt", n))
}

pub fn example_path(n: usize) -> PathBuf {
    EXAMPLES_DIR.join(format!("{}.txt", n))
}

pub fn example_exists(n: usize) -> bool {
    example_path(n).exists()
}

pub fn solution_exists(n: usize) -> bool {
    solution_path(n).exists()
}

pub mod prelude {
    pub use crate::iter::IteratorExt;
    pub use crate::number::UnsignedExt;
    pub use crate::parsing::U8PtrExt;
    pub use crate::point::PointExt;
    pub use crate::range::RangeExt;
    pub use crate::slice::SliceExt;
    pub use crate::string::{CharExt, StrExt};
    pub use crate::u8::U8SliceExt;
}
