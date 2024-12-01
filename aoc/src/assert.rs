use std::{fmt::Debug, str::FromStr};

use crate::*;

pub fn assert_solution<T: FromStr + Eq + Debug>(solution: usize, solve: impl FnOnce(&str) -> T)
where
    <T as FromStr>::Err: Debug,
{
    if !solution_exists(solution) {
        return;
    }

    let mut example = solution;
    for i in (0..solution).rev() {
        if example_exists(example) {
            break;
        }
        example = i;
    }
    if !example_exists(example) {
        panic!("No example found for solution {}", solution);
    }

    let expected = read_solution_to_string(solution).parse::<T>().unwrap();
    let actual = solve(&read_example_to_string(example));
    assert_eq!(expected, actual);
}

pub fn assert_solution_mut<T: FromStr + Eq + Debug>(
    solution: usize,
    solve: impl FnOnce(&mut str) -> T,
) where
    <T as FromStr>::Err: Debug,
{
    if !solution_exists(solution) {
        return;
    }

    let mut example = solution;
    for i in (0..solution).rev() {
        if example_exists(example) {
            break;
        }
        example = i;
    }
    if !example_exists(example) {
        panic!("No example found for solution {}", solution);
    }

    let expected = read_solution_to_string(solution).parse::<T>().unwrap();
    let actual = solve(&mut read_example_to_string(example));
    assert_eq!(expected, actual);
}
