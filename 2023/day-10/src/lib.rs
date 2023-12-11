mod tile;
use tile::*;

pub fn part_1(input: &mut str) -> usize {
    let grid = Grid::from_ascii_str(input);
    let start = grid.find_start();

    // // let mut pgrid = vec![vec!['.'; grid.width]; grid.height];
    // // pgrid[start.row as usize][start.col as usize] = 'S';

    let (a, b) = grid.connected_neighbours(start);
    let mut walker_1 = Walker::new(start, a);
    let mut walker_2 = Walker::new(start, b);

    // // pgrid[walker_1.current.row as usize][walker_1.current.col as usize] =
    // //     tile_to_unicode_tile(grid.tile_at(&walker_1.current).unwrap());
    // // pgrid[walker_2.current.row as usize][walker_2.current.col as usize] =
    // //     tile_to_unicode_tile(grid.tile_at(&walker_2.current).unwrap());

    let mut step = 1;
    loop {
        walker_1.step(&grid);
        walker_2.step(&grid);

        //     // pgrid[walker_1.current.row as usize][walker_1.current.col as usize] =
        //     //     tile_to_unicode_tile(grid.tile_at(&walker_1.current).unwrap());

        //     // pgrid[walker_2.current.row as usize][walker_2.current.col as usize] =
        //     //     tile_to_unicode_tile(grid.tile_at(&walker_2.current).unwrap());

        step += 1;

        if walker_1.current == walker_2.current {
            break;
        }
    }

    // // pgrid[walker_1.current.row as usize][walker_1.current.col as usize] = '1';
    // // pgrid[walker_2.current.row as usize][walker_2.current.col as usize] = '2';

    // // pgrid.iter().for_each(|line| {
    // //     line.iter().for_each(|c| print!("{}", c));
    // //     println!();
    // // });

    step
}

pub fn part_2(tiles: &mut str) -> usize {
    let mut grid = Grid::from_ascii_str(tiles);
    let start = grid.find_start();

    let (a, b) = grid.connected_neighbours(start);
    let mut walker_1 = Walker::new(start, a);
    let mut walker_2 = Walker::new(start, b);

    // unsafe { std::mem::transmute::<&mut [u8], &mut [Tile]>(grid.tiles) }
    //     .chunks_exact(4)
    //     .for_each(|chunk| println!("{:?}", chunk));

    // let mut step = 1;
    loop {
        walker_1.step(&grid);
        walker_2.step(&grid);

        grid.mark_tile(walker_1.prev);
        grid.mark_tile(walker_2.prev);

        // step += 1;

        if walker_1.current == walker_2.current {
            break;
        }
    }
    // println!("Steps: {}", step);

    grid.mark_tile(walker_1.current);

    // println!("Marked:");
    // unsafe { std::mem::transmute::<&mut [u8], &mut [Tile]>(grid.tiles) }
    //     .chunks_exact(grid.width)
    //     .for_each(|chunk| {
    //         println!(
    //             "{}",
    //             chunk
    //                 .iter()
    //                 .map(|t| t.to_unicode_char())
    //                 .collect::<String>()
    //         )
    //     });

    let mut count = 0;
    for row in 0..grid.height {
        let mut intersections = 0;
        for col in 0..grid.width {
            let pos = (row * grid.width) + col;
            let c: Tile = unsafe { std::mem::transmute(grid.tiles[pos]) };
            match c {
                Tile::NSMarked | Tile::NEMarked | Tile::NWMarked => {
                    intersections += 1;
                }
                Tile::EWMarked
                | Tile::SEMarked
                | Tile::SWMarked
                | Tile::StartMArked
                | Tile::Start => {}
                _ => {
                    if intersections % 2 == 1 {
                        count += 1;
                        // println!("{}: {}", count, pos);
                    }
                }
            }
        }
    }

    // println!("Count: {}", count);

    // println!("{}", grid.tiles);

    count
}

#[derive(Debug)]
struct Walker {
    pub current: i32,
    pub prev: i32,
    pub dir: Direction,
}

impl Walker {
    fn new(prev: i32, start: (i32, Direction)) -> Self {
        Self {
            prev,
            current: start.0,
            dir: start.1,
        }
    }

    fn step(&mut self, grid: &Grid) -> i32 {
        let (next, new_dir) = grid.next_connected_neighbour(self.current, self.dir);
        self.dir = new_dir;
        self.prev = self.current;
        self.current = next;
        self.current
    }
}

#[derive(Debug)]
struct Grid<'a> {
    pub tiles: &'a mut [u8],
    pub width: usize,
    pub height: usize,
}

impl<'g> Grid<'g> {
    fn from_ascii_str(tiles: &'g mut str) -> Self {
        let row_len = tiles.find('\n').unwrap();
        unsafe {
            tiles
                .as_bytes_mut()
                .iter_mut()
                .for_each(|t| *t = Tile::from_ascii_char(*t as char) as u8);

            Self {
                tiles: tiles.as_bytes_mut(),
                width: row_len + 1,
                height: row_len,
            }
        }
    }

    fn connected_neighbours(&self, pos: i32) -> ((i32, Direction), (i32, Direction)) {
        let (a, dir_a) = self.next_connected_neighbour(pos, Direction::NONE);
        let (b, dir_b) = self.next_connected_neighbour(pos, dir_a.opposite());

        ((a, dir_a), (b, dir_b))
    }

    fn next_connected_neighbour(&self, pos: i32, dir: Direction) -> (i32, Direction) {
        let tile = self.tile_at(pos).unwrap();

        if !(dir == Direction::DOWN) {
            let above = pos - self.width as i32;
            if let Some(p) = self.tile_at(above) {
                if tile.connects_to(p, Direction::UP) {
                    return (above, Direction::UP);
                }
            }
        }

        if !(dir == Direction::UP) {
            let below = pos + self.width as i32;
            if let Some(p) = self.tile_at(below) {
                if tile.connects_to(p, Direction::DOWN) {
                    return (below, Direction::DOWN);
                }
            }
        }

        if !(dir == Direction::RIGHT) {
            let left = pos - 1;
            if let Some(p) = self.tile_at(left) {
                if tile.connects_to(p, Direction::LEFT) {
                    return (left, Direction::LEFT);
                }
            }
        }

        if !(dir == Direction::LEFT) {
            let right = pos + 1;
            if let Some(p) = self.tile_at(right) {
                if tile.connects_to(p, Direction::RIGHT) {
                    return (right, Direction::RIGHT);
                }
            }
        }

        panic!("No neighbours found for {:?}", pos);
    }

    fn find_start(&self) -> i32 {
        self.tiles
            .iter()
            .position(|&p| p == Tile::Start as u8)
            .unwrap() as i32
    }

    fn tile_at(&self, idx: i32) -> Option<Tile> {
        if idx < 0 || idx >= self.tiles.len() as i32 {
            return None;
        }

        unsafe { Some(std::mem::transmute(self.tiles[idx as usize])) }
    }

    fn mark_tile(&mut self, idx: i32) {
        unsafe {
            self.tiles[idx as usize] =
                *std::mem::transmute::<u8, Tile>(self.tiles[idx as usize]).mark() as u8;
        }
    }
}

pub fn tile_to_unicode_tile(c: char) -> char {
    match c {
        '|' => '│',
        '-' => '─',
        'L' => '└',
        'J' => '┘',
        'F' => '┌',
        '7' => '┐',
        _ => c,
    }
}
