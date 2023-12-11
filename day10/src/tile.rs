use std::fmt::Display;

use crate::location::{Location, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Represents a single tile in the map. (1 char in the puzzle input)
/// 
/// Variants:
/// - [`Vectical`][`Tile::Vertical`]: Has connections up and down
///     (Representedby `|`)
/// - [`Horizontal`][`Tile::Horizontal`]: Has connections left and right
///     (Represented by `-`)
/// - [`Corner`][`Tile::Corner`]: Has connections in 2 directions
///     (represented by `J`, `L`, `7`, and `F` \[see [`CornerType`][`crate::tile::CornerType`]\])
/// - [`Empty`][`Tile::Empty`]: Has no connections (represented by `.`)
/// - [`StartingPosition`][`Tile::StartingPosition`]: The starting position.
///     See the variant for more info.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    /// Has connections up and down (Represented by `|`)
    Vertical,
    /// Has connections left and right (Represented by `-`)
    Horizontal,
    /// Has connections in 2 directions (represented by `J`, `L`, `7`, and `F` \[see [`CornerType`][`crate::tile::CornerType`]\])
    Corner(CornerType),
    /// Has no connections (represented by `.`)
    Empty,
    /// The starting position (represented by `S`)
    /// (May have connections in any direction, but in practice 2.)
    /// 
    /// The tile at the starting position is not specified in the input file, so
    /// [`Loops`][`crate::pipe_loop::Loop`] figures out what it represents on
    /// the fly.
    StartingPosition,
}



impl Tile {
    /// Get the locations that this tile connects to.
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

    /// Given the 2 connections that this tile connects 2, return the
    /// [`Horizontal`][Tile::Horizontal], [`Vertical`][Tile::Vertical], or
    /// [`Corner`][Tile::Corner] variant that fits that description.
    pub fn from_directions(a: Direction, b: Direction) -> Self {
        use Tile::*;
        use crate::tile::CornerType::*;

        // Simple sort so that the logic can be more compact.
        let (a, b) = if a < b { (a, b) } else { (b, a) };

        match (a, b) {
            // Has above connection
            (Direction::Up, Direction::Left) => Corner(TopLeft),
            (Direction::Up, Direction::Right) => Corner(TopRight),
            (Direction::Up, Direction::Down) => Vertical,

            // Has left connection
            (Direction::Left, Direction::Right) => Horizontal,
            (Direction::Left, Direction::Down) => Corner(BottomLeft),

            // Has right connection
            _ => Corner(BottomRight),
        }
    }

    /// Assuming that this tile is located at `at`, returns true if it connects
    /// to `to`. (See [`Tile::get_connections()`])
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
