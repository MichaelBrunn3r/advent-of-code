const CHAR_TO_PIPE_MAP: [Tile; 128] = generate_char_to_pipe_map();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tile {
    //     In   Out
    //     ablr udlr
    /** '|' */
    NS = 0b1100_1100,
    /** '─' */
    EW = 0b0011_0011,
    /** '└' */
    NE = 0b1001_1001,
    /** '┘' */
    NW = 0b1010_1010,
    /** '┌' */
    SE = 0b0101_0101,
    /** '┐' */
    SW = 0b0110_0110,
    /** 'S' */
    Start = 0b1111_1111,
    /** '.' */
    Ground = 0b0000_0000,

    /** '|' */
    NSMarked = 0b1100_1110,
    /** '─' */
    EWMarked = 0b0011_1110,
    /** '└' */
    NEMarked = 0b1001_1110,
    /** '┘' */
    NWMarked = 0b1010_1110,
    /** '┌' */
    SEMarked = 0b0101_1110,
    /** '┐' */
    SWMarked = 0b0110_1110,
    /** 'S' */
    StartMArked = 0b1111_1110,
}

impl Tile {
    pub fn from_ascii_char(c: char) -> Self {
        CHAR_TO_PIPE_MAP[c as usize]
    }

    pub fn connects_to(&self, other: Tile, dir: Direction) -> bool {
        let self_has_out = (*self as u8) & (dir as u8 & 0b0000_1111);
        let other_has_in = other as u8 & (dir as u8 & 0b1111_0000);
        self_has_out > 0 && other_has_in > 0
    }

    #[inline(always)]
    pub fn mark(tile: u8) -> u8 {
        (tile | 0b1110) & 0b1111_1110
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
            Tile::Start | Tile::StartMArked => 'S',
            Tile::Ground => '.',
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    //     In   Out
    //     ablr udlr
    UP = 0b0100_1000,
    DOWN = 0b1000_0100,
    LEFT = 0b0001_0010,
    RIGHT = 0b0010_0001,
    NONE = 0b0000_0000,
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
    fn test_can_go_from_to() {
        let tests = vec![
            // │
            (Tile::NS, Tile::NS, vec![UP, DOWN], vec![LEFT, RIGHT]),
            (Tile::NS, Tile::EW, vec![], vec![UP, DOWN, LEFT, RIGHT]),
            (Tile::NS, Tile::NE, vec![DOWN], vec![UP, LEFT, RIGHT]),
            (Tile::NS, Tile::NW, vec![DOWN], vec![UP, LEFT, RIGHT]),
            (Tile::NS, Tile::SE, vec![UP], vec![DOWN, LEFT, RIGHT]),
            (Tile::NS, Tile::SW, vec![UP], vec![DOWN, LEFT, RIGHT]),
            // ─
            (Tile::EW, Tile::NS, vec![], vec![UP, DOWN, LEFT, RIGHT]),
            (Tile::EW, Tile::EW, vec![LEFT, RIGHT], vec![UP, DOWN]),
            (Tile::EW, Tile::NE, vec![LEFT], vec![UP, DOWN, RIGHT]),
            (Tile::EW, Tile::NW, vec![RIGHT], vec![UP, DOWN, LEFT]),
            (Tile::EW, Tile::SE, vec![LEFT], vec![UP, DOWN, RIGHT]),
            (Tile::EW, Tile::SW, vec![RIGHT], vec![UP, DOWN, LEFT]),
            // └
            (Tile::NE, Tile::NS, vec![UP], vec![DOWN, LEFT, RIGHT]),
            (Tile::NE, Tile::EW, vec![RIGHT], vec![UP, DOWN, LEFT]),
            (Tile::NE, Tile::NE, vec![], vec![UP, DOWN, LEFT, RIGHT]),
            (Tile::NE, Tile::NW, vec![RIGHT], vec![UP, DOWN, LEFT]),
            (Tile::NE, Tile::SE, vec![UP], vec![DOWN, LEFT, RIGHT]),
            (Tile::NE, Tile::SW, vec![UP, RIGHT], vec![DOWN, LEFT]),
            // ┘
            (Tile::NW, Tile::NS, vec![UP], vec![DOWN, LEFT, RIGHT]),
            (Tile::NW, Tile::EW, vec![LEFT], vec![UP, DOWN, RIGHT]),
            (Tile::NW, Tile::NE, vec![LEFT], vec![UP, DOWN, RIGHT]),
            (Tile::NW, Tile::NW, vec![], vec![UP, DOWN, LEFT, RIGHT]),
            (Tile::NW, Tile::SE, vec![UP, LEFT], vec![DOWN, RIGHT]),
            (Tile::NW, Tile::SW, vec![UP], vec![DOWN, LEFT, RIGHT]),
            // ┌
            (Tile::SE, Tile::NS, vec![DOWN], vec![UP, LEFT, RIGHT]),
            (Tile::SE, Tile::EW, vec![RIGHT], vec![UP, DOWN, LEFT]),
            (Tile::SE, Tile::NE, vec![DOWN], vec![UP, LEFT, RIGHT]),
            (Tile::SE, Tile::NW, vec![DOWN, RIGHT], vec![UP, LEFT]),
            (Tile::SE, Tile::SE, vec![], vec![UP, DOWN, LEFT, RIGHT]),
            (Tile::SE, Tile::SW, vec![RIGHT], vec![UP, DOWN, LEFT]),
            // ┐
            (Tile::SW, Tile::NS, vec![DOWN], vec![UP, LEFT, RIGHT]),
            (Tile::SW, Tile::EW, vec![LEFT], vec![UP, DOWN, RIGHT]),
            (Tile::SW, Tile::NE, vec![DOWN, LEFT], vec![UP, RIGHT]),
            (Tile::SW, Tile::NW, vec![DOWN], vec![UP, LEFT, RIGHT]),
            (Tile::SW, Tile::SE, vec![LEFT], vec![UP, DOWN, RIGHT]),
            (Tile::SW, Tile::SW, vec![], vec![UP, DOWN, LEFT, RIGHT]),
        ];

        for (from, to, possible, impossible) in tests {
            for dir in possible {
                assert!(from.connects_to(to, dir));
            }
            for dir in impossible {
                assert!(!from.connects_to(to, dir));
            }
        }
    }
}
