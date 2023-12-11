use std::fmt::Debug;
use std::ops::Index;

use crate::{tile::{ Tile, CornerType }, location::Location};

/// A map of all of the pipe tiles.
/// 
/// Use:
/// - [`Map::parse()`] to parse a map from the puzzle input.
/// - [`Map::get_starting_location()`] to get the starting location of the map.
/// - [`Map::in_bounds()`] to check if a location is within the bounds of the map.
/// - [`Map::location_iter()`] to iterate through all locations in the map.
/// - [`Map::width()`] and [`Map::height()`] to get the dimensions of the map.
#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    /// Parses a map from a list of strings (the lines of the puzzle input).
    pub fn parse(lines: &[&str]) -> Self {
        let mut tiles = Vec::new();

        for line in lines {
            let mut row = Vec::new();
            for c in line.chars() {
                let tile = match c {
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    'L' => Tile::Corner(CornerType::TopRight),
                    'J' => Tile::Corner(CornerType::TopLeft),
                    '7' => Tile::Corner(CornerType::BottomLeft),
                    'F' => Tile::Corner(CornerType::BottomRight),
                    '.' => Tile::Empty,
                    'S' => Tile::StartingPosition,
                    _ => panic!("Unknown tile: {}", c),
                };
                row.push(tile);
            }
            tiles.push(row);
        }

        Self { tiles }
    }

    /// Returns the starting location of the map.
    /// 
    /// Panics if no starting location is found.
    pub fn get_starting_location(&self) -> Location {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::StartingPosition {
                    return Location { x, y };
                }
            }
        }
        panic!("No starting location found");
    }

    /// Returns true if the location is within the bounds of the map.
    pub fn in_bounds(&self, location: Location) -> bool {
        location.x < self.width() && location.y < self.height()
    }

    /// Iterator through all locations in the map.
    /// 
    /// Top to bottom, left to right. ("reading order")
    pub fn location_iter(&self) -> impl Iterator<Item = Location> {
        // Anli, this is literally the one time I had to fight the borrow
        // checker, and  the solution was 1 simple line.
        let height = self.height();

        (0..self.width()).flat_map(move |x| (0..height).map(move |y| Location { x, y }))
    }

    /// Number of columns in the map
    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    /// Number of rows in the map
    pub fn height(&self) -> usize {
        self.tiles.len()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<Location> for Map {
    type Output = Tile;

    fn index(&self, location: Location) -> &Self::Output {
        &self.tiles[location.y][location.x]
    }
}
