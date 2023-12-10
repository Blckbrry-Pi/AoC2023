use std::fmt::Display;

use crate::location::{Location, Direction};

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

    pub fn from_directions(a: Direction, b: Direction) -> Self {
        use Tile::*;
        use crate::tile::CornerType::*;

        // Simple sort so that the logic can be more compact.
        let (a, b) = if a < b { (a, b) } else { (b, a) };

        match (a, b) {
            // Has above connection
            (Direction::Above, Direction::Left) => Corner(TopLeft),
            (Direction::Above, Direction::Right) => Corner(TopRight),
            (Direction::Above, Direction::Below) => Vertical,

            // Has left connection
            (Direction::Left, Direction::Right) => Horizontal,
            (Direction::Left, Direction::Below) => Corner(BottomLeft),

            // Has right connection
            _ => Corner(BottomRight),
        }
    }

    pub fn connects(&self, at: Location, to: Location) -> bool {
        self.get_connections(at).contains(&to)
    }
}

impl Display for CornerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let corner = if f.alternate() {
            match self {
                CornerType::TopLeft => "╝",
                CornerType::TopRight => "╚",
                CornerType::BottomLeft => "╗",
                CornerType::BottomRight => "╔",
            }
        }  else {
            match self {
                CornerType::TopLeft => "J",
                CornerType::TopRight => "L",
                CornerType::BottomLeft => "7",
                CornerType::BottomRight => "F",
            }
        };
        write!(f, "{}", corner)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = if f.alternate() {
            match self {
                Tile::Vertical => "║",
                Tile::Horizontal => "═",
                Tile::Corner(corner) => return corner.fmt(f),
                Tile::Empty => " ",
                Tile::StartingPosition => "╳",
            }
        } else {
            match self {
                Tile::Vertical => "|",
                Tile::Horizontal => "-",
                Tile::Corner(corner) => return corner.fmt(f),
                Tile::Empty => " ",
                Tile::StartingPosition => "S",
            }
        };
        write!(f, "{}", tile)
    }
}
