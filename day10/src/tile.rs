use std::fmt::Display;

use crate::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Vertical,
    Horizontal,
    Corner(CornerType),
    Empty,
    StartingPosition,
}

impl Tile {
    pub fn get_connections(&self, loc: Location) -> Vec<Location> {
        match self {
            Tile::Vertical => vec![loc.up(), loc.down()],
            Tile::Horizontal => vec![loc.left(), loc.right()],
            Tile::Corner(corner) => match corner {
                CornerType::TopLeft => vec![loc.up(), loc.left()],
                CornerType::TopRight => vec![loc.up(), loc.right()],
                CornerType::BottomLeft => vec![loc.down(), loc.left()],
                CornerType::BottomRight => vec![loc.down(), loc.right()],
            },
            Tile::Empty => Vec::new(),
            Tile::StartingPosition => vec![loc.up(), loc.left(), loc.right(), loc.down()],
        }
    }
}

impl Display for CornerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let corner = match self {
            CornerType::TopLeft => "J",
            CornerType::TopRight => "L",
            CornerType::BottomLeft => "7",
            CornerType::BottomRight => "F",
        };
        write!(f, "{}", corner)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = match self {
            Tile::Vertical => "|",
            Tile::Horizontal => "-",
            Tile::Corner(corner) => return write!(f, "{corner}"),
            Tile::Empty => " ",
            Tile::StartingPosition => "S",
        };
        write!(f, "{}", tile)
    }
}
