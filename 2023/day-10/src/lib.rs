use std::{path, str::FromStr};

use aoc::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn part_1(input: &mut str) -> usize {
    let grid = Grid::from_tiles(input);
    let start = grid.find_start();

    // let mut pgrid = vec![vec!['.'; grid.width]; grid.height];
    // pgrid[start.row as usize][start.col as usize] = 'S';

    let (a, b) = grid.connected_neighbours(&start);
    let mut walker_1 = Walker::new(start, a);
    let mut walker_2 = Walker::new(start, b);

    // pgrid[walker_1.current.row as usize][walker_1.current.col as usize] =
    //     tile_to_unicode_tile(grid.tile_at(&walker_1.current).unwrap());
    // pgrid[walker_2.current.row as usize][walker_2.current.col as usize] =
    //     tile_to_unicode_tile(grid.tile_at(&walker_2.current).unwrap());

    let mut step = 1;
    loop {
        walker_1.step(&grid);
        walker_2.step(&grid);

        // pgrid[walker_1.current.row as usize][walker_1.current.col as usize] =
        //     tile_to_unicode_tile(grid.tile_at(&walker_1.current).unwrap());

        // pgrid[walker_2.current.row as usize][walker_2.current.col as usize] =
        //     tile_to_unicode_tile(grid.tile_at(&walker_2.current).unwrap());

        step += 1;

        if walker_1.current == walker_2.current {
            break;
        }
    }

    // pgrid[walker_1.current.row as usize][walker_1.current.col as usize] = '1';
    // pgrid[walker_2.current.row as usize][walker_2.current.col as usize] = '2';

    // pgrid.iter().for_each(|line| {
    //     line.iter().for_each(|c| print!("{}", c));
    //     println!();
    // });

    step
}

pub fn part_2(tiles: &mut str) -> usize {
    let mut grid = Grid::from_tiles(tiles);
    let start = grid.find_start();

    let (a, b) = grid.connected_neighbours(&start);
    let mut walker_1 = Walker::new(start, a);
    let mut walker_2 = Walker::new(start, b);

    loop {
        walker_1.step(&grid);
        walker_2.step(&grid);

        grid.mark_tile(&walker_1.prev);
        grid.mark_tile(&walker_2.prev);

        if walker_1.current == walker_2.current {
            break;
        }
    }

    grid.mark_tile(&walker_1.current);

    // println!("{}", grid.tiles);

    let mut count = 0;
    for row in 0..grid.height {
        // println!("row={}", row + 1);
        let mut intersections = 0;
        let mut last_intersection = 0u8;
        for col in 0..grid.width {
            let pos = (row * grid.width) + col;
            let c = grid.tiles[pos];
            match c {
                b'{' | b'E' | b'K' => {
                    intersections += 1;
                    last_intersection = c;
                    // println!("{}: {}", intersections, pos);
                }
                b'6' => {
                    if last_intersection == b'E' {
                        intersections += 1;
                        // println!("{}: {}", intersections, pos);
                    }
                    last_intersection = c;
                }
                b'I' => {
                    if last_intersection == b'K' {
                        intersections += 1;
                        // println!("{}: {}", intersections, pos);
                    }
                    last_intersection = c;
                }
                b',' | b'Q' | b'S' => {}
                _ => {
                    if intersections % 2 == 1 {
                        count += 1;
                    }

                    if c == b'.' {
                        grid.tiles[pos] = if intersections % 2 == 1 { b'i' } else { b'o' };
                    }
                }
            }
        }
    }

    // println!("{}", grid.tiles);

    count
}

#[derive(Debug)]
struct Walker {
    pub current: Position,
    pub prev: Position,
    pub dir: Direction,
}

impl Walker {
    fn new(prev: Position, start: (Position, Direction)) -> Self {
        Self {
            prev,
            current: start.0,
            dir: start.1,
        }
    }

    fn step(&mut self, grid: &Grid) -> Position {
        let (next, new_dir) = grid.next_connected_neighbour(&self.current, self.dir);
        self.dir = new_dir;
        self.prev = self.current;
        self.current = next;
        self.current
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Initial,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Initial => Self::Initial,
        }
    }
}

#[derive(Debug)]
struct Grid<'a> {
    pub tiles: &'a mut [u8],
    pub width: usize,
    pub height: usize,
}

impl<'g> Grid<'g> {
    fn from_tiles(tiles: &'g mut str) -> Self {
        let row_len = tiles.find('\n').unwrap();
        unsafe {
            Self {
                tiles: tiles.as_bytes_mut(),
                width: row_len + 1,
                height: row_len,
            }
        }
    }

    fn connected_neighbours(
        &self,
        pos: &Position,
    ) -> ((Position, Direction), (Position, Direction)) {
        let (a, dir_a) = self.next_connected_neighbour(pos, Direction::Initial);
        let (b, dir_b) = self.next_connected_neighbour(pos, dir_a.opposite());

        ((a, dir_a), (b, dir_b))
    }

    fn next_connected_neighbour(&self, pos: &Position, dir: Direction) -> (Position, Direction) {
        let tile = self.tile_at(&pos).unwrap();

        if !(dir == Direction::Down) {
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
                | ('S', Some('|')) => return (above, Direction::Up),
                _ => {}
            }
        }

        if !(dir == Direction::Up) {
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
                | ('S', Some('|'))
                | (_, Some('S')) => return (below, Direction::Down),
                _ => {}
            }
        }

        if !(dir == Direction::Right) {
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
                | (Some('-'), 'S')
                | (Some('S'), _) => return (left, Direction::Left),
                _ => {}
            }
        }

        if !(dir == Direction::Left) {
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
                | ('S', Some('-'))
                | (_, Some('S')) => return (right, Direction::Right),
                _ => {}
            }
        }

        panic!("No neighbours found for {:?}", pos);
    }

    fn find_start(&self) -> Position {
        let start = self.tiles.iter().position(|&c| c == b'S').unwrap();
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

        Some(self.tiles[row * self.width + col] as char)
    }

    fn mark_tile(&mut self, pos: &Position) {
        // | -> {
        // - -> ,
        // L -> K
        // J -> I
        // 7 -> 6
        // F -> E
        // S -> R
        let row = pos.row as usize;
        let col = pos.col as usize;

        self.tiles[row * self.width + col] -= 1;
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
