use std::collections::HashSet;

use aoc::prelude::*;
use itertools::Itertools;

// b'A' = 0100_0001, b'Z' = 0101_1010
// Flags: ??VPPPPP, [P]lant, [V]isited

const MASK_PLANT: u8 = 0b0001_1111;
const FLAG_VISITED: u8 = 0b0010_0000;
const LINE_LENGTH: usize = 5;

pub fn p1(input: &mut str) -> usize {
    let bytes = unsafe { input.as_bytes_mut() };
    let line_length = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;
    let bytes_len = bytes.len();

    let mut total = 0;

    let mut unhandled_plots = vec![0usize];
    while let Some(idx_start_plot) = unhandled_plots.pop() {
        let plant = bytes[idx_start_plot];
        if plant & FLAG_VISITED != 0 || plant == b'\n' {
            continue;
        }
        println!("{:?}", plant as char);
        bytes[idx_start_plot] |= FLAG_VISITED;

        let mut plot_area = 0;
        let mut stack = vec![idx_start_plot];
        let mut edge = HashSet::new();
        while let Some(idx_current) = stack.pop() {
            plot_area += 1;
            for idx_adjacent in [1i32, -1, line_length as i32, -(line_length as i32)]
                .iter()
                .map(|&offset| idx_current as i32 + offset)
            {
                if idx_adjacent < 0 || idx_adjacent >= bytes_len as i32 {
                    edge.insert(idx_current);
                    continue;
                }

                let idx_adjacent = idx_adjacent as usize;
                let adjacent = bytes[idx_adjacent];

                if adjacent == b'\n' {
                    edge.insert(idx_current);
                    continue;
                }

                if (plant & MASK_PLANT) == (adjacent & MASK_PLANT) {
                    if adjacent & FLAG_VISITED == 0 {
                        stack.push(idx_adjacent);
                    }
                    bytes[idx_adjacent] |= FLAG_VISITED;
                } else {
                    edge.insert(idx_current);
                    if adjacent & FLAG_VISITED == 0 {
                        unhandled_plots.push(idx_adjacent);
                    }
                }
            }
        }

        let mut edge = edge
            .drain()
            .map(|idx| (idx % LINE_LENGTH, idx / LINE_LENGTH, idx))
            .collect_vec();
        edge.sort_by_key(|e| (e.1, e.0));
        // println!("  {:?}", edge);

        let mut prev_has_top_edge = false;
        let mut prev_has_bottom_edge = false;
        let mut sides_h: usize = edge
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                let mut sum = 0;
                if b.1 != a.1 {
                    sum += prev_has_top_edge as usize + prev_has_bottom_edge as usize;
                }

                if b.2 - a.2 != 1 {
                    return 0;
                }

                prev_has_top_edge = is_edge(plant, b.2 as i32 - LINE_LENGTH as i32, bytes);
                if is_edge(plant, a.2 as i32 - LINE_LENGTH as i32, bytes)
                    != prev_has_top_edge
                {
                    sum += 1;
                }

                prev_has_bottom_edge = is_edge(plant, b.2 as i32 + LINE_LENGTH as i32, bytes);
                if is_edge(plant, a.2 as i32 + LINE_LENGTH as i32, bytes)
                    != prev_has_bottom_edge
                {
                    sum += 1;
                }

                sum
            })
            .sum::<usize>();
        if prev_has_top_edge {
            sides_h += 1;
        }
        if prev_has_bottom_edge {
            sides_h += 1;
        }
        if sides_h == 0 && edge.len() == 1 {
            sides_h = 2;
        }

        edge.sort_by_key(|e| (e.0, e.1));
        let mut sides_v: usize = edge
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                if b.2 as i32 - a.2 as i32 != LINE_LENGTH as i32 {
                    return 0;
                }

                let mut sum = 0;
                if is_edge(plant, a.2 as i32 - 1 as i32, bytes)
                    != is_edge(plant, b.2 as i32 - 1 as i32, bytes)
                {
                    sum += 1;
                }

                if is_edge(plant, a.2 as i32 + 1 as i32, bytes)
                    != is_edge(plant, b.2 as i32 + 1 as i32, bytes)
                {
                    sum += 1;
                }

                sum
            })
            .sum();
        if sides_v == 0 && edge.len() == 1 {
            sides_v = 2;
        }

        println!("    {sides_h} {sides_v}");
        total += plot_area * (sides_h + sides_v);
    }

    total
}

pub fn p2(input: &str) -> usize {
    0
}

const ADJACENT_EDGE_DIRS: [(i32, Direction); 4] = [
    (1, Direction::Right),                 // Right -> Right
    (LINE_LENGTH as i32, Direction::Down), // Below -> Down
    (-1, Direction::Left),                 // Left -> Left
    (LINE_LENGTH as i32, Direction::Up),   // Above -> Up
];

pub fn walk_perimiter(
    idx_start: usize,
    bytes: &mut [u8],
) -> (HashSet<i32>, Vec<usize>, usize, usize) {
    let plant = bytes[idx_start];
    let dir = ADJACENT_EDGE_DIRS.iter().find_map(|(offset, dir)| {
        let idx_adjacent = idx_start as i32 + offset;
        if !is_edge(plant, idx_adjacent, bytes) {
            return Some(dir.clone());
        }
        None
    });
    if dir.is_none() {
        println!("  sides={}\n  permimeter={:?}", 4, 1);
        bytes[idx_start] |= FLAG_VISITED;
        return (HashSet::new(), vec![], 4, 1);
    };

    let mut dir = dir.unwrap();
    let dir_start = dir;

    let mut perimeter = HashSet::new();
    let mut inside = HashSet::new();
    let mut outside = HashSet::new();
    let mut sides = 0;
    let mut idx_current = idx_start as i32;
    for _ in 0..20 {
        inside.remove(&(idx_current));
        outside.insert(idx_current + dir.edge_offset());
        if idx_current == idx_start as i32
            && dir == dir_start
            && bytes[idx_current as usize] & FLAG_VISITED != 0
        {
            // if is_edge(plant, dir.walk(idx_current), bytes) {
            //     println!("  clockwise last");
            //     sides += 1;
            // }
            break;
        }
        let idx_next = dir.walk(idx_current);
        // println!("  {idx_current}->{idx_next} {:?}", dir);

        // Clockwise corner ━━━┓
        if is_edge(plant, idx_next, bytes) {
            // println!("  clockwise");
            dir.turn_clockwise();
            sides += 1;
            continue;
        }

        if !is_edge(plant, idx_current + dir.opposite_edge_offset(), bytes) {
            if bytes[(idx_current + dir.opposite_edge_offset()) as usize] & FLAG_VISITED == 0 {
                inside.insert(idx_current + dir.opposite_edge_offset());
            }
        }

        // Counter-clockwise corner ━━━┛
        if !is_edge(plant, idx_next + dir.edge_offset(), bytes) {
            if bytes[idx_next as usize] & FLAG_VISITED == 0 {
                inside.insert(idx_next);
            }

            perimeter.insert(idx_current);

            bytes[idx_current as usize] |= FLAG_VISITED;
            idx_current = dir.walk_counter_clockwise(idx_current);
            dir.turn_counter_clockwise();
            // println!("  counter-clockwise");
            sides += 1;
            continue;
        }

        perimeter.insert(idx_current);
        bytes[idx_current as usize] |= FLAG_VISITED;
        idx_current = idx_next;
    }

    let outside = outside
        .drain()
        .filter(|&idx| idx >= 0 && idx < bytes.len() as i32 && bytes[idx as usize] != b'\n')
        .map(|idx| idx as usize)
        .collect_vec();

    println!(
        "  sides={}\n  permimeter={:?}\n  inside={:?}\n  outside={:?}",
        sides,
        perimeter.len(),
        inside,
        outside
    );
    (inside, outside, sides, perimeter.len())
}

fn is_edge(plant: u8, idx_other: i32, bytes: &[u8]) -> bool {
    idx_other < 0
        || idx_other >= bytes.len() as i32
        || (plant & MASK_PLANT) != (bytes[idx_other as usize] & MASK_PLANT)
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn walk(&self, current_idx: i32) -> i32 {
        const OFFSET: [i32; 4] = [
            1,                     // Right
            LINE_LENGTH as i32,    // Down
            -1,                    // Left
            -(LINE_LENGTH as i32), // Up
        ];

        current_idx + OFFSET[*self as usize]
    }

    fn walk_counter_clockwise(&self, current_idx: i32) -> i32 {
        const OFFSET: [i32; 4] = [
            1 - LINE_LENGTH as i32,  // Right: Right+Up
            1 + LINE_LENGTH as i32,  // Down:  Right+Down
            -1 + LINE_LENGTH as i32, // Left:  Left+Down
            -1 - LINE_LENGTH as i32, // Up:    Left+Up
        ];

        current_idx + OFFSET[*self as usize]
    }

    fn turn_clockwise(&mut self) {
        const TURN: [Direction; 4] = [
            Direction::Down,  // Right -> Down
            Direction::Left,  // Down -> Left
            Direction::Up,    // Left -> Up
            Direction::Right, // Up -> Right
        ];

        *self = TURN[*self as usize];
    }

    fn turn_counter_clockwise(&mut self) {
        const TURN: [Direction; 4] = [
            Direction::Up,    // Right -> Up
            Direction::Right, // Down -> Right
            Direction::Down,  // Left -> Up
            Direction::Left,  // Up -> Right
        ];

        *self = TURN[*self as usize];
    }

    fn edge_offset(&self) -> i32 {
        const EDGE_OFFSET: [i32; 4] = [
            -(LINE_LENGTH as i32), // Right: Top
            1,                     // Down: Right
            LINE_LENGTH as i32,    // Left: Below
            -1,                    // Up: Left
        ];
        EDGE_OFFSET[*self as usize]
    }

    fn opposite_edge_offset(&self) -> i32 {
        const EDGE_OFFSET: [i32; 4] = [
            LINE_LENGTH as i32,    // Right: Below
            -1,                    // Down: Left
            -(LINE_LENGTH as i32), // Left: Above
            1,                     // Up: Right
        ];
        EDGE_OFFSET[*self as usize]
    }
}
