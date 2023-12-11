mod tile;
use tile::*;

pub fn part_1(input: &mut str) -> usize {
    let grid = Grid::from_ascii_str(input);
    let start = grid.find_start();

    let (a, b) = grid.connected_neighbours(start);
    let mut walker_1 = Walker::new(a);
    let mut walker_2 = Walker::new(b);

    let mut step = 1;
    loop {
        walker_1.step(&grid);
        walker_2.step(&grid);

        step += 1;

        if walker_1.current == walker_2.current {
            break;
        }
    }

    step
}

pub fn part_2(input: &mut str) -> usize {
    let mut grid = Grid::from_ascii_str(input);
    let start = grid.find_start();

    let (a, b) = grid.connected_neighbours(start);
    let mut walker_1 = Walker::new(a);
    let mut walker_2 = Walker::new(b);

    while walker_1.current != walker_2.current {
        grid.mark_tile(walker_1.step(&grid));
        grid.mark_tile(walker_2.step(&grid));
    }

    grid.mark_tile(walker_1.current);

    let mut count = 0;
    for row in 0..grid.height {
        let mut intersections = 0;

        for pos in (row * grid.width)..(row * grid.width + grid.width) {
            let c: Tile = unsafe { std::mem::transmute(grid.tiles[pos]) };
            match c {
                Tile::NSMarked | Tile::NEMarked | Tile::NWMarked => {
                    intersections ^= 1;
                }
                Tile::EWMarked
                | Tile::SEMarked
                | Tile::SWMarked
                | Tile::StartMArked
                | Tile::Start => {}
                _ => {
                    if intersections == 1 {
                        count += 1;
                    }
                }
            }
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
        self.tiles[idx as usize] = (self.tiles[idx as usize] | 0b1110) & 0b1111_1110;
    }
}
