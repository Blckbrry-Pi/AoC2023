#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Open,
    Spring,
    Unknown,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Spring,
            '?' => Tile::Unknown,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Open => '.',
            Tile::Spring => '#',
            Tile::Unknown => '?',
        }
    }
}
