use std::collections::VecDeque;

use aoc::prelude::*;
use itertools::Itertools;

// 1000 too low
// 1100 too high
// 1104 too high
pub fn part_1(input: &mut str) -> usize {
    let size = input.find('\n').unwrap();

    // Convert numbers to integers
    let map = unsafe { input.as_bytes_mut() }
        .chunks_exact_mut(size + 1)
        .map(|line| line[0..size].iter().map(|b| b - b'0').collect_vec())
        .collect_vec();

    let start = (0usize, 0usize);
    let goal = (size - 1, size - 1);

    let mut paths = vec![Path::new(start)];

    let mut iter = 0;
    let mut res = None;
    while !paths.is_empty() {
        iter += 1;
        let path = paths.swap_remove(0);
        let last = *path.last();
        if last == goal {
            res = Some(path);
            break;
        }

        // Expand
        let mut expanded_paths = Vec::new();
        for i in (1..=3).rev() {
            if path.last_dir != Direction::Left && path.last_dir != Direction::Right {
                // Expand right
                if last.1 + i < size {
                    let mut p = path.clone();
                    let next = (last.0, last.1 + i);
                    // Check for loop
                    if !p.visits(&next) {
                        let cost = (1..=i).map(|j| map[last.0][last.1 + j] as usize).sum();
                        p.add(next, cost, next.manhattan_distance(goal), Direction::Right);
                        expanded_paths.push(p);
                    }
                }

                // Expand left
                if last.1 >= i {
                    let mut p = path.clone();
                    let next = (last.0, last.1 - i);
                    // Check for loop
                    if !p.visits(&next) {
                        let cost = (1..=i).map(|j| map[last.0][last.1 - j] as usize).sum();
                        p.add(
                            next,
                            cost,
                            next.manhattan_distance(goal) + 1,
                            Direction::Left,
                        );
                        expanded_paths.push(p);
                    }
                }
            }

            if path.last_dir != Direction::Up && path.last_dir != Direction::Down {
                // Expand down
                if last.0 + i < size {
                    let mut p = path.clone();
                    let next = (last.0 + i, last.1);
                    // Check for loop
                    if !p.visits(&next) {
                        let cost = (1..=i).map(|j| map[last.0 + j][last.1] as usize).sum();
                        p.add(next, cost, next.manhattan_distance(goal), Direction::Down);
                        expanded_paths.push(p);
                    }
                }

                // Expand up
                if last.0 >= i {
                    let mut p = path.clone();
                    let next = (last.0 - i, last.1);
                    // Check for loop
                    if !p.visits(&next) {
                        let cost = (1..=i).map(|j| map[last.0 - j][last.1] as usize).sum();
                        p.add(next, cost, next.manhattan_distance(goal) + 1, Direction::Up);
                        expanded_paths.push(p);
                    }
                }
            }
        }

        // If path that ends in the same node exists, keep the one with the lowest cost
        expanded_paths.retain(|p| {
            let last = *p.last();
            if let Some(other_pos) = paths.iter().position(|p| *p.last() == last) {
                let other = paths.get(other_pos).unwrap();
                return p.cost < other.cost;
            };
            true
        });

        paths.extend(expanded_paths);

        paths.sort_by_key(|p| p.cost + p.estimated_rest_cost);

        if iter % 10_000 == 0 {
            println!("Iterations: {}", iter);
            pretty_print(&map, &paths[0]);
        }
    }

    let res = res.unwrap();
    pretty_print(&map, &res);
    println!("Iterations: {}", iter);
    println!("Path: {:?}", res);

    res.cost
}

const AVG_HEAT_LOSS: usize = 5;

pub fn part_2(input: &str) -> usize {
    0
}

fn pretty_print(map: &[Vec<u8>], path: &Path) {
    let mut pretty = vec![vec![b'.'; map.len()]; map.len()];

    path.nodes.iter().tuple_windows().for_each(|(prev, curr)| {
        if prev.0 < curr.0 {
            for i in prev.0 + 1..=curr.0 {
                pretty[i][prev.1] = b'v';
            }
        } else if prev.0 > curr.0 {
            for i in curr.0..prev.0 {
                pretty[i][prev.1] = b'^';
            }
        } else if prev.1 < curr.1 {
            for i in prev.1 + 1..=curr.1 {
                pretty[prev.0][i] = b'>';
            }
        } else if prev.1 > curr.1 {
            for i in curr.1..prev.1 {
                pretty[prev.0][i] = b'<';
            }
        }
    });
    println!(
        "{}",
        pretty
            .iter()
            .map(|line| String::from_utf8_lossy(line))
            .join("\n")
    );
}

#[derive(Debug, Clone)]
struct Path {
    nodes: Vec<(usize, usize)>,
    cost: usize,
    estimated_rest_cost: usize,
    last_dir: Direction,
}

impl Path {
    fn new(start: (usize, usize)) -> Self {
        Self {
            nodes: vec![start],
            cost: 0,
            last_dir: Direction::None,
            estimated_rest_cost: 0,
        }
    }

    fn last(&self) -> &(usize, usize) {
        self.nodes.last().unwrap()
    }

    fn visits(&self, node: &(usize, usize)) -> bool {
        self.nodes.contains(node)
    }

    fn add(
        &mut self,
        node: (usize, usize),
        cost: usize,
        estimated_rest_cost: usize,
        dir: Direction,
    ) {
        self.nodes.push(node);
        self.cost += cost;
        self.estimated_rest_cost = estimated_rest_cost;
        self.last_dir = dir;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
