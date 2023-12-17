use std::fmt::Debug;

use crate::tile::Tile;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Tile::parse(c));
            }
            tiles.push(row);
        }
        Self { tiles }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

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
