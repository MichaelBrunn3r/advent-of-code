use std::collections::VecDeque;

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

// 1104 too high
pub fn part_1(input: &mut str) -> usize {
    let height = input.find('\n').unwrap();
    let width = height + 1;
    let map = unsafe { input.as_bytes_mut() };

    // Convert ASCII numbers to integers. '\n' becomes 218
    map.chunks_exact_mut(height).for_each(|line| {
        for c in line.iter_mut() {
            *c = c.wrapping_sub(b'0');
        }
    });

    let start = (0usize, 0usize);
    let goal = (height - 1, width - 2);
    let mut paths = VecDeque::from([Path::new(start)]);

    let mut iter = 0;
    let mut res = None;
    while !paths.is_empty() {
        iter += 1;
        let path = paths.pop_front().unwrap();
        let last = *path.last();
        if last == goal {
            // println!("Found path: {:?}", path);
            res = Some(path);
            break;
        }

        // Expand
        let mut expanded_paths = Vec::new();
        for i in (1..=3).rev() {
            if path.last_dir != Direction::Left && path.last_dir != Direction::Right {
                // Expand right
                if last.1 + i < width {
                    let mut p = path.clone();
                    let next = (last.0, last.1 + i);
                    // Check for loop
                    if !p.visits(&next) {
                        let heat_loss = (1..=i)
                            .map(|j| map[last.0 * width + (last.1 + j)] as usize)
                            .sum();
                        p.add(next, i, heat_loss, Direction::Right);
                        expanded_paths.push(p);
                    }
                }

                // Expand left
                if last.1 >= i {
                    let mut p = path.clone();
                    let next = (last.0, last.1 - i);
                    // Check for loop
                    if !p.visits(&next) {
                        let heat_loss = (1..=i)
                            .map(|j| map[last.0 * width + (last.1 - j)] as usize)
                            .sum();
                        p.add(next, i, heat_loss, Direction::Left);
                        expanded_paths.push(p);
                    }
                }
            }

            if path.last_dir != Direction::Up && path.last_dir != Direction::Down {
                // Expand down
                if last.0 + i < height {
                    let mut p = path.clone();
                    let next = (last.0 + i, last.1);
                    // Check for loop
                    if !p.visits(&next) {
                        let heat_loss = (1..=i)
                            .map(|j| map[(last.0 + j) * width + last.1] as usize)
                            .sum();
                        p.add(next, i, heat_loss, Direction::Down);
                        expanded_paths.push(p);
                    }
                }

                // Expand up
                if last.0 >= i {
                    let mut p = path.clone();
                    let next = (last.0 - i, last.1);
                    // Check for loop
                    if !p.visits(&next) {
                        let heat_loss = (1..=i)
                            .map(|j| map[(last.0 - j) * width + last.1] as usize)
                            .sum();
                        p.add(next, i, heat_loss, Direction::Up);
                        expanded_paths.push(p);
                    }
                }
            }
        }

        expanded_paths.retain(|p| {
            let last = *p.last();
            if let Some(other_pos) = paths.iter().position(|p| *p.last() == last) {
                let other = paths.get(other_pos).unwrap();
                return p.heat_loss < other.heat_loss;
            }
            true
        });

        paths.extend(expanded_paths);

        paths = paths
            .into_iter()
            .sorted_by_key(|p| {
                let last = *p.last();
                let dist = last.manhattan_distance(goal);
                let rest_distance = match p.last_dir {
                    Direction::Up | Direction::Left => dist + 2,
                    _ => dist,
                };
                p.heat_loss + rest_distance
            })
            .collect();

        if iter % 10_000 == 0 {
            println!("Iterations: {}", iter);
            pretty_print(&map, &paths[0], width);
        }
    }

    let res = res.unwrap();
    pretty_print(&map, &res, width);
    println!("Iterations: {}", iter);
    println!("Path: {:?}", res);

    res.heat_loss
}

const AVG_HEAT_LOSS: usize = 5;

pub fn part_2(input: &str) -> usize {
    0
}

fn pretty_print(map: &[u8], path: &Path, width: usize) {
    let mut pretty = Vec::from(
        map.iter()
            .map(|b| if *b == 218 { b'\n' } else { b'.' })
            .collect_vec(),
    );
    path.nodes.iter().tuple_windows().for_each(|(prev, curr)| {
        if prev.0 < curr.0 {
            for i in prev.0 + 1..=curr.0 {
                pretty[i * width + prev.1] = b'v';
            }
        } else if prev.0 > curr.0 {
            for i in curr.0..prev.0 {
                pretty[i * width + prev.1] = b'^';
            }
        } else if prev.1 < curr.1 {
            for i in prev.1 + 1..=curr.1 {
                pretty[prev.0 * width + i] = b'>';
            }
        } else if prev.1 > curr.1 {
            for i in curr.1..prev.1 {
                pretty[prev.0 * width + i] = b'<';
            }
        }
    });
    println!("{}", String::from_utf8(pretty).unwrap());
}

#[derive(Debug, Clone)]
struct Path {
    nodes: Vec<(usize, usize)>,
    len: usize,
    heat_loss: usize,
    last_dir: Direction,
}

impl Path {
    fn new(start: (usize, usize)) -> Self {
        Self {
            nodes: vec![start],
            len: 0,
            heat_loss: 0,
            last_dir: Direction::None,
        }
    }

    fn last(&self) -> &(usize, usize) {
        self.nodes.last().unwrap()
    }

    fn visits(&self, node: &(usize, usize)) -> bool {
        self.nodes.contains(node)
    }

    fn add(&mut self, node: (usize, usize), path_len: usize, heat_loss: usize, dir: Direction) {
        self.nodes.push(node);
        self.len += path_len;
        self.heat_loss += heat_loss;
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
