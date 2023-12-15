use std::fmt::Debug;

use crate::tile::Tile;

#[derive(Clone)]
pub struct GroupPos {
    group_size: usize,
    locations: Vec<usize>,
}

impl Debug for GroupPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for location in self.locations.iter().copied() {
            if location > 0 {
                for _ in 0..location-1 {
                    write!(f, "?")?;
                }
                write!(f, ".")?;
            }
            for _ in 0..self.group_size {
                write!(f, "#")?;
            }
            write!(f, ".")?;
            writeln!(f, "???···")?;
        }

        Ok(())
    }
}

impl GroupPos {
    pub fn new(group_size: usize, before: &[usize], after: &[usize], len: usize) -> Self {
        let min_offset = if before.is_empty() {
            0
        } else {
            before.iter().copied().map(|len| len + 1).sum::<usize>()
        };

        let max_offset = if after.is_empty() {
            len - group_size
        } else {
            len - 1 - after.iter().copied().map(|len| len + 1).sum::<usize>() - (group_size - 1)
        }.max(min_offset);
        

        Self {
            group_size,
            locations: (min_offset..=max_offset).collect(),
        }
    }

    pub fn filter(&mut self, row: &[Tile]) {
        let mut new_locations = Vec::new();

        for location in self.locations.iter().copied() {
            let mut valid = true;

            if location != 0 && row[location - 1] == Tile::Spring {
                continue;
            }


            for i in 0..self.group_size {
                if location + i >= row.len() {
                    panic!("Location {location} out of bounds for row {row:?}");
                }
                if row[location + i] == Tile::Open {
                    valid = false;
                    continue;
                }
            }
            if !valid {
                continue;
            }


            if location + self.group_size < row.len() && row[location + self.group_size] == Tile::Spring {
                continue;
            }


            new_locations.push(location);
        }

        self.locations = new_locations;
    }

    pub fn len(&self) -> usize {
        self.locations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.locations.iter().copied()
    }

    pub fn iter_group(&self) -> impl Iterator<Item = std::ops::Range<usize>> + '_ {
        self.locations.iter().copied().map(move |location| location..location + self.group_size)
    }

    pub fn iter_starting_at(&self, start: usize) -> impl Iterator<Item = usize> + '_ {
        self.locations.iter().copied().filter(move |&location| location >= start)
    }

    pub fn iter_group_starting_at(&self, start: usize) -> impl Iterator<Item = std::ops::Range<usize>> + '_ {
        self.locations.iter().copied().filter(move |&location| location >= start).map(move |location| location..location + self.group_size)
    }
}
