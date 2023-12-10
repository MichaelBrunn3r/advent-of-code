use std::{path, str::FromStr};

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &str) -> usize {
    let row_len = input.find('\n').unwrap();
    let grid = Grid {
        tiles: input,
        width: row_len + 1,
        height: row_len,
    };

    let start = grid.find_start();

    let mut pgrid = vec![vec![' '; grid.width]; grid.height];
    pgrid[start.row as usize][start.col as usize] = 'S';

    let mut prev_1 = start;
    let mut prev_2 = start;
    let (mut current_1, mut current_2) = grid.connected_neighbours(&start);

    pgrid[current_1.row as usize][current_1.col as usize] =
        tile_to_unicode_tile(grid.tile_at(&current_1).unwrap());
    pgrid[current_2.row as usize][current_2.col as usize] =
        tile_to_unicode_tile(grid.tile_at(&current_2).unwrap());

    let mut step = 1;
    loop {
        // println!("Step={}, 1={:?}, 2={:?}", step, current_1, current_2);

        let (neighbour1, neighbour2) = grid.connected_neighbours(&current_1);
        let next_1 = if neighbour1 == prev_1 {
            neighbour2
        } else {
            neighbour1
        };
        prev_1 = current_1;
        current_1 = next_1;

        let (neighbour1, neighbour2) = grid.connected_neighbours(&current_2);
        let next_2 = if neighbour1 == prev_2 {
            neighbour2
        } else {
            neighbour1
        };
        prev_2 = current_2;
        current_2 = next_2;

        pgrid[current_1.row as usize][current_1.col as usize] =
            tile_to_unicode_tile(grid.tile_at(&current_1).unwrap());

        pgrid[current_2.row as usize][current_2.col as usize] =
            tile_to_unicode_tile(grid.tile_at(&current_2).unwrap());

        step += 1;

        if current_1 == current_2 {
            break;
        }
    }

    pgrid[current_1.row as usize][current_1.col as usize] = '1';
    pgrid[current_2.row as usize][current_2.col as usize] = '2';

    pgrid.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });

    step
}

pub fn part_2(input: &str) -> usize {
    0
}

#[derive(Debug)]
struct Grid<'a> {
    tiles: &'a str,
    width: usize,
    height: usize,
}

impl Grid<'_> {
    fn connected_neighbours(&self, pos: &Position) -> (Position, Position) {
        let mut neighbours = Vec::new();
        let tile = self.tile_at(&pos).unwrap();

        let above = pos.above();
        match (tile, self.tile_at(&above)) {
            ('|', Some('F'))
            | ('|', Some('7'))
            | ('|', Some('|'))
            | ('|', Some('S'))
            | ('L', Some('F'))
            | ('L', Some('7'))
            | ('L', Some('|'))
            | ('L', Some('S'))
            | ('J', Some('F'))
            | ('J', Some('7'))
            | ('J', Some('|'))
            | ('J', Some('S'))
            | ('S', Some('F'))
            | ('S', Some('7'))
            | ('S', Some('|')) => neighbours.push(above),
            (_, Some('.')) => {}
            _ => {}
        }

        let below = pos.below();
        match (tile, self.tile_at(&below)) {
            ('|', Some('L'))
            | ('|', Some('J'))
            | ('|', Some('|'))
            | ('|', Some('S'))
            | ('7', Some('L'))
            | ('7', Some('J'))
            | ('7', Some('|'))
            | ('7', Some('S'))
            | ('F', Some('L'))
            | ('F', Some('J'))
            | ('F', Some('|'))
            | ('F', Some('S'))
            | ('S', Some('L'))
            | ('S', Some('J'))
            | ('S', Some('|')) => neighbours.push(below),
            (_, Some('.')) => {}
            (_, Some('S')) => neighbours.push(below),
            _ => {}
        }

        let left = pos.left();
        match (self.tile_at(&left), tile) {
            (Some('L'), '-')
            | (Some('F'), '-')
            | (Some('-'), '-')
            | (Some('S'), '-')
            | (Some('L'), 'J')
            | (Some('F'), 'J')
            | (Some('-'), 'J')
            | (Some('S'), 'J')
            | (Some('L'), '7')
            | (Some('F'), '7')
            | (Some('-'), '7')
            | (Some('S'), '7')
            | (Some('L'), 'S')
            | (Some('F'), 'S')
            | (Some('-'), 'S') => neighbours.push(left),
            (Some('.'), _) => {}
            (Some('S'), _) => neighbours.push(left),
            _ => {}
        }

        let right = pos.right();
        match (tile, self.tile_at(&right)) {
            ('-', Some('J'))
            | ('-', Some('7'))
            | ('-', Some('-'))
            | ('-', Some('S'))
            | ('L', Some('J'))
            | ('L', Some('7'))
            | ('L', Some('-'))
            | ('L', Some('S'))
            | ('F', Some('J'))
            | ('F', Some('7'))
            | ('F', Some('-'))
            | ('F', Some('S'))
            | ('S', Some('J'))
            | ('S', Some('7'))
            | ('S', Some('-')) => neighbours.push(right),
            (_, Some('.')) => {}
            (_, Some('S')) => neighbours.push(right),
            _ => {}
        }

        (neighbours[0], neighbours[1])
    }

    fn find_start(&self) -> Position {
        let start = self.tiles.find('S').unwrap();
        Position::new((start % (self.width)) as i32, (start / self.width) as i32)
    }

    fn tile_at(&self, pos: &Position) -> Option<char> {
        if pos.row < 0
            || pos.row >= self.height as i32
            || pos.col < 0
            || pos.col >= self.width as i32
        {
            return None;
        }
        let row = pos.row as usize;
        let col = pos.col as usize;

        Some(self.tiles.as_bytes()[row * self.width + col] as char)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    col: i32,
    row: i32,
}

impl Position {
    fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }

    fn above(&self) -> Self {
        Self::new(self.col, self.row - 1)
    }

    fn below(&self) -> Self {
        Self::new(self.col, self.row + 1)
    }

    fn left(&self) -> Self {
        Self::new(self.col - 1, self.row)
    }

    fn right(&self) -> Self {
        Self::new(self.col + 1, self.row)
    }
}

pub fn tile_to_unicode_tile(c: char) -> char {
    match c {
        '|' => '│',
        '-' => '─',
        'L' => '└',
        'J' => '┘',
        '7' => '┐',
        'F' => '┌',
        _ => c,
    }
}
