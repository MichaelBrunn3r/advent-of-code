use std::collections::{HashMap, HashSet};

use aoc::prelude::*;
use itertools::Itertools;

const SIDE_LENGTH: usize = 50;
type NodeLocations = HashMap<char, HashSet<(usize, usize)>>;

pub fn parse(input: &str) -> NodeLocations {
    let mut node_locations = HashMap::new();
    input
        .split("\n")
        .enumerate()
        .for_each(|(y, line)| {
            line
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &c)| c != b'.')
                .for_each(|(x, &c)| {
                    let key = c as char;
                    if !node_locations.contains_key(&key) {
                        node_locations.insert(key, HashSet::new());
                    }

                    node_locations.get_mut(&key).unwrap().insert((x,y));
                });
        });

    node_locations
}   

pub fn p1(node_locations: &NodeLocations) -> usize {
    let mut antinode_locations = HashSet::new();
    node_locations
        .keys()
        .for_each(|&key| {
            let locations = node_locations.get(&key).unwrap();
            locations
                .iter()
                .cartesian_product(locations)
                .for_each(|(&a, &b)| {
                    if a == b {
                        return;
                    }

                    let dx = b.0 as i32 - a.0 as i32;
                    let dy = b.1 as i32 - a.1 as i32;

                    let a1 = (b.0 as i32 + dx, b.1 as i32 + dy);
                    let a2 = (a.0 as i32 - dx, a.1 as i32 - dy);

                    if a1.0 >= 0 && a1.0 < SIDE_LENGTH as i32 && a1.1 >= 0 && a1.1 < SIDE_LENGTH as i32 {
                        antinode_locations.insert(a1);
                    }

                    if a2.0 >= 0 && a2.0 < SIDE_LENGTH as i32 && a2.1 >= 0 && a2.1 < SIDE_LENGTH as i32 {
                        antinode_locations.insert(a2);
                    }
                });
        });
    // println!("{:?}", antinode_locations);

    antinode_locations.len()
}

pub fn p2(node_locations: &NodeLocations) -> usize {
    let mut antinode_locations = HashSet::new();
    node_locations
        .keys()
        .for_each(|&key| {
            let locations = node_locations.get(&key).unwrap();
            locations
                .iter()
                .cartesian_product(locations)
                .for_each(|(&a, &b)| {
                    if a == b {
                        return;
                    }

                    let dx = b.0 as i32 - a.0 as i32;
                    let dy = b.1 as i32 - a.1 as i32;

                    let a1 = (b.0 as i32 + dx, b.1 as i32 + dy);
                    let a2 = (a.0 as i32 - dx, a.1 as i32 - dy);

                    {
                        let mut x = b.0 as i32 + dx;
                        let mut y = b.1 as i32 + dy;
                        while x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32 {
                            antinode_locations.insert((x, y));

                            x += dx;
                            y += dy;
                        }
                    }

                    {
                        let mut x = b.0 as i32 - dx;
                        let mut y = b.1 as i32 - dy;
                        while x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32 {
                            antinode_locations.insert((x, y));

                            x -= dx;
                            y -= dy;
                        }
                    }
                });
        });

    antinode_locations.len()
}
