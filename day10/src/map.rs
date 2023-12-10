use std::fmt::Debug;
use std::ops::Index;

use crate::{tile::{ Tile, CornerType }, location::Location};

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
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

impl Map {
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

    pub fn in_bounds(&self, location: Location) -> bool {
        let x_in_bounds = location.x < self.tiles[0].len();
        let y_in_bounds = location.y < self.tiles.len();
        x_in_bounds && y_in_bounds
    }

    pub fn location_iter(&self) -> impl Iterator<Item = Location> {
        let width = self.tiles[0].len();
        let height = self.tiles.len();

        (0..width).flat_map(move |x| (0..height).map(move |y| Location { x, y }))
    }
}

impl Index<Location> for Map {
    type Output = Tile;

    fn index(&self, location: Location) -> &Self::Output {
        &self.tiles[location.y][location.x]
    }
}
