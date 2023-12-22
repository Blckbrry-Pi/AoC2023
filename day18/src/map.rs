use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::pos::Pos;
use crate::color::Color;
use crate::line::Line;
use crate::dir::Direction;
use crate::superchunk::Superchunk;


// const SUPERCHUNK_SIZE: isize = 2048;

#[derive(Clone)]
pub struct Map {
    dug_tiles: HashMap<Pos, Color>,
    curr_pos: Pos,
}

impl Map {
    pub fn new(start: Pos) -> Self {
        Self {
            dug_tiles: HashMap::new(),
            curr_pos: start,
        }
    }

    pub fn exec_line(&mut self, line: Line) {
        for _ in 0..line.length() {
            let new_pos = match line.direction() {
                Direction::U => self.curr_pos.up(),
                Direction::D => self.curr_pos.down(),
                Direction::L => self.curr_pos.left(),
                Direction::R => self.curr_pos.right(),
            };

            self.dug_tiles.insert(new_pos, line.color());
            self.curr_pos = new_pos;
        }
    }

    pub fn get(&self, pos: Pos) -> Option<Color> {
        self.dug_tiles.get(&pos).copied()
    }

    pub fn len(&self) -> usize {
        self.dug_tiles.len()
    }
    pub fn is_empty(&self) -> bool {
        self.dug_tiles.is_empty()
    }

    pub fn count_internal(&self) -> usize {
        let (min, max) = Pos::iter_range(self.dug_tiles.keys().copied());
        let (min_x, max_x) = (min.x - 2, max.x + 2);
        let (min_y, max_y) = (min.y - 2, max.y + 2);

        (max_y - min_y + 1) as usize * (max_x - min_x + 1) as usize - self.get_external(HashSet::new(), None).len()
    }

    pub fn get_external(&self, no_touch_tiles: HashSet<Pos>, starting_tiles: Option<HashSet<Pos>>) -> HashSet<Pos> {
        let (min, max) = Pos::iter_range(self.dug_tiles.keys().copied());
        let (min_x, max_x) = (min.x - 2, max.x + 2);
        let (min_y, max_y) = (min.y - 2, max.y + 2);

        let mut definitely_external = HashSet::new();
        let mut external_queue = HashSet::new();
        if let Some(starting_tiles) = starting_tiles {
            for pos in starting_tiles {
                external_queue.insert(pos);
                definitely_external.insert(pos);
            }
        } else {
            for x in min_x..=max_x {
                let pos = Pos::new(x, min_y);
                if !self.dug_tiles.contains_key(&pos) || no_touch_tiles.contains(&pos) {
                    external_queue.insert(pos);
                    definitely_external.insert(pos);
                }
                let pos = Pos::new(x, max_y);
                if !self.dug_tiles.contains_key(&pos) || no_touch_tiles.contains(&pos) {
                    external_queue.insert(pos);
                    definitely_external.insert(pos);
                }
            }
            for y in min_y..=max_y {
                let pos = Pos::new(min_x, y);
                if !self.dug_tiles.contains_key(&pos) || no_touch_tiles.contains(&pos) {
                    external_queue.insert(pos);
                    definitely_external.insert(pos);
                }
                let pos = Pos::new(max_x, y);
                if !self.dug_tiles.contains_key(&pos) || no_touch_tiles.contains(&pos) {
                    external_queue.insert(pos);
                    definitely_external.insert(pos);
                }
            }
        }

        while let Some(key) = external_queue.iter().next().copied() {
            if external_queue.len() % 1000 == 0 {
                // println!("{}", external_queue.len());
            }
            external_queue.remove(&key);
            for dir in [Direction::U, Direction::D, Direction::L, Direction::R].iter() {
                let neighbor = match dir {
                    Direction::U => if key.y <= min_y { continue; } else { key.up() },
                    Direction::D => if key.y >= max_y { continue; } else { key.down() },
                    Direction::L => if key.x <= min_x { continue; } else { key.left() },
                    Direction::R => if key.x >= max_x { continue; } else { key.right() },
                };
                if !self.dug_tiles.contains_key(&neighbor) && !definitely_external.contains(&neighbor) && !no_touch_tiles.contains(&neighbor) {
                    external_queue.insert(neighbor);
                    definitely_external.insert(neighbor);
                }
            }
        }

        definitely_external
    }


    pub fn superchunked(&self, superchunk_size: isize) -> Self {
        let tiles = self.dug_tiles
            .keys()
            .map(|pos| (Pos::new(pos.x / superchunk_size, pos.y / superchunk_size), Color::new(0, 0, 0)))
            .collect();
        Self {
            dug_tiles: tiles,
            curr_pos: Pos::new(self.curr_pos.x / superchunk_size, self.curr_pos.y / superchunk_size),
        }
    }


    pub fn visualize(tiles: HashSet<Pos>) -> Self {
        Self {
            dug_tiles: tiles.into_iter().map(|key| (key, Color::new(0, 0, 0))).collect(),
            curr_pos: Pos::new(0, 0),
        }
    }

    pub fn taken_tiles(&self) -> impl Iterator<Item = Pos> + '_ {
        self.dug_tiles.keys().copied()
    }

    pub fn range(&self) -> (Pos, Pos) {
        Pos::iter_range(self.dug_tiles.keys().copied())
    }

    pub fn offset(self, offset: Pos) -> Self {
        let dug_tiles = self.dug_tiles.into_iter()
            .map(|(pos, color)| (pos + offset, color))
            .collect();
        let curr_pos = self.curr_pos + offset;
        Self { dug_tiles, curr_pos }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dug_tiles.is_empty() {
            return Ok(())
        }
        let (min, max) = Pos::iter_range(self.dug_tiles.keys().copied());
        let (min_x, max_x) = (min.x, max.x);
        let (min_y, max_y) = (min.y, max.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Pos::new(x, y);
                if pos == Pos::new(0, 0) {
                    write!(f, "X")?;
                } else if self.dug_tiles.get(&pos).is_some() {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

