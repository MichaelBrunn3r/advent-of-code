pub mod tile;
use tile::*;

pub fn parse(input: String) -> Grid {
    Grid::from_ascii_str(input.as_bytes().to_vec())
}

pub fn part_1(grid: &Grid) -> usize {
    let start = grid.find_start();

    let (a, b) = grid.connected_neighbours(start);
    let mut walker_1 = Walker::new(a);
    let mut walker_2 = Walker::new(b);

    let mut step = 1;
    loop {
        walker_1.step(grid);
        walker_2.step(grid);

        step += 1;

        if walker_1.current == walker_2.current {
            break;
        }
    }

    step
}

pub fn part_2(grid: &mut Grid) -> usize {
    let start = grid.find_start();

    let (a, b) = grid.connected_neighbours(start);
    let mut walker_1 = Walker::new(a);
    let mut walker_2 = Walker::new(b);

    while walker_1.current != walker_2.current {
        grid.mark_tile(walker_1.step(grid));
        grid.mark_tile(walker_2.step(grid));
    }

    grid.mark_tile(walker_1.current);

    let mut count = 0;
    for row in 0..grid.height {
        let mut is_inside = 0;

        for pos in (row * grid.width)..(row * grid.width + grid.width) {
            let tile = grid.tiles[pos];
            let is_marked = Tile::is_marked(tile) as u8;
            let is_north_facing = Tile::is_north_facing(tile) as u8;
            let is_not_start = (tile != Tile::Start as u8) as u8;

            is_inside ^= is_north_facing & is_marked;
            count += (!is_marked & is_not_start & is_inside) as usize;
        }
    }
    count
}

#[derive(Debug)]
struct Walker {
    pub current: i32,
    pub dir: Direction,
}

impl Walker {
    fn new(start: (i32, Direction)) -> Self {
        Self {
            current: start.0,
            dir: start.1,
        }
    }

    fn step(&mut self, grid: &Grid) -> i32 {
        let (next, new_dir) = grid.next_connected_neighbour(self.current, self.dir);
        self.dir = new_dir;
        let prev = self.current;
        self.current = next;
        prev
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub tiles: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn from_ascii_str(mut tiles: Vec<u8>) -> Self {
        let row_len = 140;
        tiles
            .iter_mut()
            .for_each(|t| *t = Tile::from_ascii_char(*t as char) as u8);

        Self {
            tiles,
            width: row_len + 1,
            height: row_len,
        }
    }

    fn connected_neighbours(&self, pos: i32) -> ((i32, Direction), (i32, Direction)) {
        let mut neighbours = Vec::new();

        let above = pos - self.width as i32;
        let below = pos + self.width as i32;
        let left = pos - 1;
        let right = pos + 1;

        if let Some(tile) = self.tile_at(above) {
            if tile.can_enter_with(Direction::UP) {
                neighbours.push((above, Direction::UP));
            }
        }
        if let Some(tile) = self.tile_at(below) {
            if tile.can_enter_with(Direction::DOWN) {
                neighbours.push((below, Direction::DOWN));
            }
        }
        if let Some(tile) = self.tile_at(left) {
            if tile.can_enter_with(Direction::LEFT) {
                neighbours.push((left, Direction::LEFT));
            }
        }
        if let Some(tile) = self.tile_at(right) {
            if tile.can_enter_with(Direction::RIGHT) {
                neighbours.push((right, Direction::RIGHT));
            }
        }

        (neighbours[0], neighbours[1])
    }

    fn next_connected_neighbour(&self, mut pos: i32, dir: Direction) -> (i32, Direction) {
        let tile = self.tile_at(pos).unwrap();

        let next_dir = tile.next_dir(dir);
        pos -= (next_dir == Direction::UP) as i32 * self.width as i32;
        pos += (next_dir == Direction::DOWN) as i32 * self.width as i32;
        pos -= (next_dir == Direction::LEFT) as i32;
        pos += (next_dir == Direction::RIGHT) as i32;

        (pos, next_dir)
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
        self.tiles[idx as usize] = Tile::mark(self.tiles[idx as usize]);
    }
}
