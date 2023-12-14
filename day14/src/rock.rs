use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Empty,
    CubeRock,
    RoundRock,
}

impl TileType {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::CubeRock,
            'O' => Self::RoundRock,
            _ => panic!("Invalid tile type"),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Empty => '.',
            Self::CubeRock => '#',
            Self::RoundRock => 'O',
        }
    }
}

impl Debug for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Write::write_char(f, self.to_char())
    }
}
