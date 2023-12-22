use std::{fmt::Debug, collections::{HashSet, HashMap}};

use crate::{tile::Tile, position::Position, step::Step};

#[derive(Clone, PartialEq, Eq)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{tile:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let tiles = input.lines().map(|line| line.chars().map(Tile::parse).collect()).collect();
        Self {
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn start_pos(&self) -> Option<Position> {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::GardenPlot { is_start: true } = tile {
                    return Some(Position::new(x as isize, y as isize));
                }
            }
        }
        None
    }

    pub fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.width() as isize || y >= self.height() as isize {
            return None;
        }

        let row = &self.tiles[y as usize];
        Some(row[x as usize])
    }

    pub fn get_pos(&self, pos: Position) -> Option<Tile> {
        self.get(pos.x(), pos.y())
    }

    pub fn take_steps(&self, start: HashSet<Position>, steps: impl Iterator<Item = Step>) -> HashSet<Position> {
        let mut new_positions = HashSet::new();
        for step in steps {
            new_positions.extend(
                start
                    .iter()
                    .map(|pos| *pos + step)
                    .filter(|pos| self.get_pos(*pos).is_some())
                    .filter(|pos| self.get_pos(*pos) != Some(Tile::Rock))
            );
        }
        new_positions
    }

    pub fn take_steps_modular(&self, start: HashMap<Position, HashSet<Position>>, steps: impl Iterator<Item = Step>) -> HashMap<Position, HashSet<Position>> {
        let mut new_positions = HashMap::new();
        for step in steps {
            for (&base_pos, offsets) in start.iter() {
                let (new_base_pos, offset_step) = self.mod_position(base_pos + step);

                if !matches!(self.get_pos(new_base_pos), Some(Tile::GardenPlot { .. })) {
                    continue;
                }

                new_positions
                    .entry(new_base_pos)
                    .or_insert_with(HashSet::new)
                    .extend(offsets.iter().map(|offset| *offset + offset_step));
                    // .extend(std::iter::once(Position::new(0, 0)));
            }
        }
        new_positions
    }

    pub fn mod_position(&self, pos: Position) -> (Position, Position) {
        let w = self.width() as isize;
        let h = self.height() as isize;

        let x = pos.x().rem_euclid(w);
        let y = pos.y().rem_euclid(h);
        
        let x_offset = (x - pos.x()) / w;
        let y_offset = (y - pos.y()) / h;

        (Position::new(x, y), Position::new(x_offset, y_offset))
    }
}
