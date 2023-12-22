use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    GardenPlot { is_start: bool },
    Rock,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::GardenPlot { is_start: true } => write!(f, "S"),
            Tile::GardenPlot { is_start: false } => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

impl Tile {
    pub fn parse(c: char) -> Self {
        match c {
            'S' => Tile::GardenPlot { is_start: true },
            '.' => Tile::GardenPlot { is_start: false },
            '#' => Tile::Rock,
            _ => panic!("Unexpected character: {c}"),
        }
    }
}
