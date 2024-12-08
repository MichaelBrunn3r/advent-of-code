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

pub fn p(node_locations: &NodeLocations) -> (usize, usize) {
    let mut antinode_locations = HashSet::new();
    let mut antinode_locations_harmonics = HashSet::new();

    node_locations
        .keys()
        .for_each(|&key| {
            let locations = Vec::from_iter(node_locations.get(&key).unwrap().iter());
            for i in 0..locations.len() {
                for j in i+1..locations.len() {
                    let a = locations[i];
                    let b = locations[j];

                    antinode_locations_harmonics.insert((a.0 as i32, a.1 as i32));
                    antinode_locations_harmonics.insert((b.0 as i32, b.1 as i32));

                    let dx = b.0 as i32 - a.0 as i32;
                    let dy = b.1 as i32 - a.1 as i32;

                    {
                        let mut x = b.0 as i32 + dx;
                        let mut y = b.1 as i32 + dy;

                        if x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32 {
                            antinode_locations.insert((x,y));
                        }

                        while x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32 {
                            antinode_locations_harmonics.insert((x, y));

                            x += dx;
                            y += dy;
                        }
                    }

                    {
                        let mut x = a.0 as i32 - dx;
                        let mut y = a.1 as i32 - dy;

                        if x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32 {
                            antinode_locations.insert((x,y));
                        }

                        while x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32 {
                            antinode_locations_harmonics.insert((x, y));

                            x -= dx;
                            y -= dy;
                        }
                    }
                }
            }
        });

    (antinode_locations.len(), antinode_locations_harmonics.len())
}
