use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::pos::Pos;
use crate::color::Color;
use crate::line::Line;
use crate::dir::Direction;


const SUPERCHUNK_SIZE: isize = 2048;

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
        let min_x = self.dug_tiles.keys().map(|pos| pos.x).min().unwrap() - 2;
        let min_y = self.dug_tiles.keys().map(|pos| pos.y).min().unwrap() - 2;
        let max_x = self.dug_tiles.keys().map(|pos| pos.x).max().unwrap() + 2;
        let max_y = self.dug_tiles.keys().map(|pos| pos.y).max().unwrap() + 2;

        (max_y - min_y + 1) as usize * (max_x - min_x + 1) as usize - self.get_external(HashSet::new(), None).len()
    }

    pub fn get_external(&self, no_touch_tiles: HashSet<Pos>, starting_tiles: Option<HashSet<Pos>>) -> HashSet<Pos> {
        let min_x = self.dug_tiles.keys().chain(no_touch_tiles.iter()).map(|pos| pos.x).min().unwrap() - 2;
        let min_y = self.dug_tiles.keys().chain(no_touch_tiles.iter()).map(|pos| pos.y).min().unwrap() - 2;
        let max_x = self.dug_tiles.keys().chain(no_touch_tiles.iter()).map(|pos| pos.x).max().unwrap() + 2;
        let max_y = self.dug_tiles.keys().chain(no_touch_tiles.iter()).map(|pos| pos.y).max().unwrap() + 2;

        println!("x: {min_x} to {max_x}");
        println!("y: {min_y} to {max_y}");

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

    pub fn get_external_superchunks(&self, no_touch_tiles: HashSet<Pos>, starting_tiles: Option<HashSet<Pos>>, superchunk_size: isize) -> HashSet<Pos> {
        let superchunked = self.superchunked(superchunk_size);

        superchunked.get_external(no_touch_tiles, starting_tiles)
    }

    pub fn estimate_internal(&self, superchunk_size: isize) -> usize {
        let min_x = self.dug_tiles.keys().map(|pos| pos.x).min().unwrap();
        let min_y = self.dug_tiles.keys().map(|pos| pos.y).min().unwrap();
        let max_x = self.dug_tiles.keys().map(|pos| pos.x).max().unwrap();
        let max_y = self.dug_tiles.keys().map(|pos| pos.y).max().unwrap();

        let min_x_superchunk = min_x / superchunk_size - 2;
        let min_y_superchunk = min_y / superchunk_size - 2;
        let max_x_superchunk = max_x / superchunk_size + 2;
        let max_y_superchunk = max_y / superchunk_size + 2;

        let tiles_verti = max_y_superchunk - min_y_superchunk + 1;
        let tiles_horiz = max_x_superchunk - min_x_superchunk + 1;

        let (tiles_verti, tiles_horiz) = (tiles_verti as usize, tiles_horiz as usize);
        let superchunk_tiles = self.get_external_superchunks(HashSet::new(), None, superchunk_size).len();

        (tiles_verti * tiles_horiz - superchunk_tiles) * superchunk_size as usize * superchunk_size as usize
    }

    pub fn cap_range(&self, tiles: HashSet<Pos>) -> HashSet<Pos> {
        let min_x = self.dug_tiles.keys().map(|pos| pos.x).min().unwrap() - 2;
        let min_y = self.dug_tiles.keys().map(|pos| pos.y).min().unwrap() - 2;
        let max_x = self.dug_tiles.keys().map(|pos| pos.x).max().unwrap() + 2;
        let max_y = self.dug_tiles.keys().map(|pos| pos.y).max().unwrap() + 2;
        
        tiles.into_iter()
            .filter(|pos| min_x <= pos.x && pos.x <= max_x)
            .filter(|pos| min_y <= pos.y && pos.y <= max_y)
            .collect()
    }
    pub fn cap_range_superchunks(&self, tiles: HashSet<Pos>, superchunk_size: isize) -> HashSet<Pos> {
        let min_x = self.dug_tiles.keys().map(|pos| pos.x).min().unwrap() / superchunk_size - 2;
        let min_y = self.dug_tiles.keys().map(|pos| pos.y).min().unwrap() / superchunk_size - 2;
        let max_x = self.dug_tiles.keys().map(|pos| pos.x).max().unwrap() / superchunk_size + 2;
        let max_y = self.dug_tiles.keys().map(|pos| pos.y).max().unwrap() / superchunk_size + 2;
        
        tiles.into_iter()
            .filter(|pos| min_x <= pos.x / superchunk_size && pos.x / superchunk_size <= max_x)
            .filter(|pos| min_y <= pos.y / superchunk_size && pos.y / superchunk_size <= max_y)
            .collect()
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

    pub fn get_superchunked_border(&self, external: HashSet<Pos>, superchunk_size: isize) -> (HashSet<Pos>, HashSet<Pos>) {
        let superchunked_self = self.superchunked(superchunk_size);

        let mut external_border = HashSet::new();
        let mut internal_border = HashSet::new();


        for &pos in superchunked_self.dug_tiles.keys() {
            let pos_l = Direction::L.step(pos);
            if external.contains(&pos_l) {
                for i in 0..superchunk_size {
                    let external_border_pos = Pos::new(pos.x * superchunk_size - 1, pos_l.y * superchunk_size + i);
                    let internal_border_pos = Pos::new(pos.x * superchunk_size, pos_l.y * superchunk_size + i);
                    external_border.insert(external_border_pos);
                    internal_border.insert(internal_border_pos);
                }
            }

            let pos_r = Direction::R.step(pos);
            if external.contains(&pos_r) {
                for i in 0..superchunk_size {
                    let external_border_pos = Pos::new(pos.x * superchunk_size + superchunk_size, pos_r.y * superchunk_size + i);
                    let internal_border_pos = Pos::new(pos.x * superchunk_size + superchunk_size - 1, pos_r.y * superchunk_size + i);
                    external_border.insert(external_border_pos);
                    internal_border.insert(internal_border_pos);
                }
            }
            
            let pos_u = Direction::U.step(pos);
            if external.contains(&pos_u) {
                for i in 0..superchunk_size {
                    let external_border_pos = Pos::new(pos_u.x * superchunk_size + i, pos.y * superchunk_size - 1);
                    let internal_border_pos = Pos::new(pos_u.x * superchunk_size + i, pos.y * superchunk_size);
                    external_border.insert(external_border_pos);
                    internal_border.insert(internal_border_pos);
                }
            }

            let pos_d = Direction::D.step(pos);
            if external.contains(&pos_d) {
                for i in 0..superchunk_size {
                    let external_border_pos = Pos::new(pos_d.x * superchunk_size + i, pos.y * superchunk_size + superchunk_size);
                    let internal_border_pos = Pos::new(pos_d.x * superchunk_size + i, pos.y * superchunk_size + superchunk_size - 1);
                    external_border.insert(external_border_pos);
                    internal_border.insert(internal_border_pos);
                }
            }
        }

        let internal_border = internal_border.difference(&self.dug_tiles.keys().copied().collect()).copied().collect();

        // println!("a");
        
        (internal_border, external_border)
    }

    pub fn visualize(tiles: HashSet<Pos>) -> Self {
        Self {
            dug_tiles: tiles.into_iter().map(|key| (key, Color::new(0, 0, 0))).collect(),
            curr_pos: Pos::new(0, 0),
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dug_tiles.is_empty() {
            return Ok(())
        }
        let min_x = self.dug_tiles.keys().map(|pos| pos.x).min().unwrap();
        let min_y = self.dug_tiles.keys().map(|pos| pos.y).min().unwrap();
        let max_x = self.dug_tiles.keys().map(|pos| pos.x).max().unwrap();
        let max_y = self.dug_tiles.keys().map(|pos| pos.y).max().unwrap();

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

