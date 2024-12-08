use std::collections::{HashMap, HashSet};

use aoc::prelude::*;
use itertools::Itertools;

const SIDE_LENGTH: usize = 50;
const LINE_LENGTH: usize = SIDE_LENGTH+1;
type NodeLocations = Vec<Vec<(usize, usize)>>;

pub fn parse(input: &str) -> NodeLocations {
    let mut node_locations = vec![Vec::new(); (b'z' - b'0' + 1) as usize];

    input
        .as_bytes()
        .chunks_exact(LINE_LENGTH)
        .enumerate()
        .for_each(|(y, line)| {
            line[..SIDE_LENGTH]
                .iter()
                .enumerate()
                .filter(|(_, &c)| c != b'.')
                .for_each(|(x, &c)| {
                    node_locations[(c - b'0') as usize].push((x,y));
                });
        });

    node_locations
}   

pub fn p(node_locations: &NodeLocations) -> (usize, usize) {
    let mut antinode_locations = [0u64; SIDE_LENGTH];
    let mut antinode_locations_harmonics = [0u64; SIDE_LENGTH];

    node_locations
        .iter()
        .for_each(|node_locations| {
            node_locations
                .iter()
                .tuple_combinations()
                .for_each(|(&a, &b)| {
                    antinode_locations_harmonics[a.1] |= 1 << a.0;
                    antinode_locations_harmonics[b.1] |= 1 << b.0;

                    let dx = b.0 as i32 - a.0 as i32;
                    let dy = b.1 as i32 - a.1 as i32;

                    {
                        let mut x = b.0 as i32 + dx;
                        let mut y = b.1 as i32 + dy;

                        if in_bounds(x,y) {
                            antinode_locations[y as usize] |= 1 << x;
                            loop {
                                antinode_locations_harmonics[y as usize] |= 1 << x;
    
                                x += dx;
                                y += dy;
    
                                if !in_bounds(x,y) {
                                    break;
                                }
                            }
                        }
                    }

                    {
                        let mut x = a.0 as i32 - dx;
                        let mut y = a.1 as i32 - dy;

                        if in_bounds(x,y) {
                            antinode_locations[y as usize] |= 1 << x;
                            loop {
                                antinode_locations_harmonics[y as usize] |= 1 << x;
    
                                x -= dx;
                                y -= dy;
    
                                if !in_bounds(x,y) {
                                    break;
                                }
                            }
                        }


                    }
                });
        });

    (
        antinode_locations.iter().map(|bitmap| bitmap.count_ones() as usize).sum(),
        antinode_locations_harmonics.iter().map(|bitmap| bitmap.count_ones() as usize).sum(),
    )
}

fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < SIDE_LENGTH as i32 && y >= 0 && y < SIDE_LENGTH as i32
}