const CHAR_TO_PIPE_MAP: [Tile; 128] = generate_char_to_pipe_map();
const NEXT_DIR_MAP: [u8; 16] = generate_next_dir_map();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tile {
    /** '|' */
    NS = 0b1100,
    /** '─' */
    EW = 0b0011,
    /** '└' */
    NE = 0b1001,
    /** '┘' */
    NW = 0b1010,
    /** '┌' */
    SE = 0b0101,
    /** '┐' */
    SW = 0b0110,
    /** 'S' */
    Start = 0b0000,
    /** '.' */
    Ground = 0b0111,

    /** '|' */
    NSMarked = 0b1111_1100,
    /** '─' */
    EWMarked = 0b1111_0011,
    /** '└' */
    NEMarked = 0b1111_1001,
    /** '┘' */
    NWMarked = 0b1111_1010,
    /** '┌' */
    SEMarked = 0b1111_0101,
    /** '┐' */
    SWMarked = 0b1111_0110,
    /** 'S' */
    StartMarked = 0b1111_0000,
}

impl Tile {
    pub fn from_ascii_char(c: char) -> Self {
        CHAR_TO_PIPE_MAP[c as usize]
    }

    #[inline(always)]
    pub fn mark(tile: u8) -> u8 {
        tile | 0b1111_0000
    }

    #[inline(always)]
    pub fn is_unmarked(tile: u8) -> bool {
        tile & 0b1111_0000 == 0
    }

    #[inline(always)]
    pub fn is_marked(tile: u8) -> bool {
        tile & 0b1111_0000 == 0b1111_0000
    }

    pub fn is_north_facing(tile: u8) -> bool {
        tile & 0b0000_1000 == 0b0000_1000
    }

    pub fn to_unicode_char(&self) -> char {
        match self {
            Tile::NS => '│',
            Tile::NSMarked => '┃',
            Tile::EW => '─',
            Tile::EWMarked => '━',
            Tile::NE => '└',
            Tile::NEMarked => '┗',
            Tile::NW => '┘',
            Tile::NWMarked => '┛',
            Tile::SE => '┌',
            Tile::SEMarked => '┏',
            Tile::SW => '┐',
            Tile::SWMarked => '┓',
            Tile::Start | Tile::StartMarked => 'S',
            Tile::Ground => '.',
        }
    }

    #[inline(always)]
    pub fn next_dir(&self, dir: Direction) -> Direction {
        unsafe { std::mem::transmute(NEXT_DIR_MAP[((*self as u8) ^ (dir as u8)) as usize]) }
    }

    pub fn can_enter_with(&self, dir: Direction) -> bool {
        self.next_dir(dir) != Direction::NONE
    }
}

const fn generate_char_to_pipe_map() -> [Tile; 128] {
    let mut map = [Tile::Ground; 128];
    map[b'|' as usize] = Tile::NS;
    map[b'-' as usize] = Tile::EW;
    map[b'J' as usize] = Tile::NW;
    map[b'L' as usize] = Tile::NE;
    map[b'F' as usize] = Tile::SE;
    map[b'7' as usize] = Tile::SW;
    map[b'S' as usize] = Tile::Start;
    map
}

const fn generate_next_dir_map() -> [u8; 16] {
    let mut map = [0; 16];
    map[0b0001] = 0b0010;
    map[0b0010] = 0b0001;
    map[0b0100] = 0b1000;
    map[0b1000] = 0b0100;
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    UP = 0b0100,
    DOWN = 0b1000,
    LEFT = 0b0001,
    RIGHT = 0b0010,
    NONE = 0b0000,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::NONE => Direction::NONE,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const UP: Direction = Direction::UP;
    const DOWN: Direction = Direction::DOWN;
    const LEFT: Direction = Direction::LEFT;
    const RIGHT: Direction = Direction::RIGHT;

    #[test]
    fn test_next_dir() {
        let tests = vec![
            (Tile::NS, UP, UP),
            (Tile::NS, DOWN, DOWN),
            (Tile::EW, LEFT, LEFT),
            (Tile::EW, RIGHT, RIGHT),
            (Tile::NE, DOWN, RIGHT),
            (Tile::NE, LEFT, UP),
            (Tile::NW, DOWN, LEFT),
            (Tile::NW, RIGHT, UP),
            (Tile::SE, UP, RIGHT),
            (Tile::SE, LEFT, DOWN),
            (Tile::SW, UP, LEFT),
            (Tile::SW, RIGHT, DOWN),
        ];

        for (tile, dir, expected) in tests {
            assert_eq!(tile.next_dir(dir), expected);
        }
    }
}
