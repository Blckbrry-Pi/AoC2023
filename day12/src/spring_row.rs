use std::fmt::Debug;

use crate::{tile::Tile, group::GroupPos};

#[derive(Clone, PartialEq)]
pub struct Row {
    pub tiles: Vec<Tile>,
    pub groups: Vec<usize>,
}

impl Row {
    pub fn group_positions(&self) -> Vec<GroupPos> {
        let mut group_positions = Vec::new();

        for i in 0..self.groups.len() {
            group_positions.push(GroupPos::new(self.groups[i], &self.groups[..i], &self.groups[i + 1..], self.tiles.len()));
        }

        group_positions
    }

    pub fn parse(line: &str) -> Self {
        let (tile_str, group_str) = line.split_once(' ').unwrap();

        let tiles: Vec<_> = tile_str.chars().map(Tile::from).collect();
        let groups = group_str
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        
        Self { tiles, groups }
    }

    pub fn render_possibility(&self, possibility: &[usize]) -> Row {
        assert_eq!(possibility.len(), self.groups.len());

        let mut output = vec![Tile::Open; self.tiles.len()];

        for (len, idx) in self.groups.iter().copied().zip(possibility.iter().copied()) {
            output[idx..idx+len].copy_from_slice(&vec![Tile::Spring; len]);
        }

        Self { tiles: output, groups: self.groups.to_vec() }
    }

    pub fn expand(&self) -> Self {
        let mut tiles = Vec::with_capacity(self.tiles.len() * 5 + 4);
        for i in 0..5 {
            tiles.extend_from_slice(&self.tiles);
            if i != 4 { tiles.push(Tile::Unknown) }
        }

        let groups: Vec<_> = (0..5).flat_map(|_| self.groups.iter().copied()).collect();

        Self { tiles, groups }
    }

    pub fn is_valid(&self) -> Option<bool> {
        if self.tiles.iter().any(|t| *t == Tile::Unknown) {
            return None;
        }

        let mut seen_groups = vec![];
        let mut curr_group_size = 0;

        for &tile in &self.tiles {
            if tile == Tile::Spring {
                curr_group_size += 1;
            } else if curr_group_size > 0 {
                seen_groups.push(curr_group_size);
                curr_group_size = 0;
            }
        }
        if curr_group_size > 0 {
            seen_groups.push(curr_group_size);
        }

        Some(seen_groups == self.groups)
    }

    pub fn is_partially_invalid(&self) -> bool {
        let mut group_idx = 0;
        let mut curr_group_size = 0;

        for &tile in &self.tiles {
            if tile == Tile::Spring {
                curr_group_size += 1;
                if curr_group_size > self.groups[group_idx] {
                    return true;
                }
            } else if tile == Tile::Open {
                if curr_group_size > 0 {
                    if curr_group_size != self.groups[group_idx] {
                        return true;
                    }

                    group_idx += 1;
                    curr_group_size = 0;
                }
            } else if tile == Tile::Unknown {
                return curr_group_size > self.groups[group_idx];
            }
        }
        if curr_group_size > 0 {
            curr_group_size != self.groups[group_idx]
        } else {
            false
        }
    }

    pub fn valid_iter(&mut self) -> impl Iterator<Item = Self> + '_ {
        if let Some(valid) = self.is_valid() {
            if valid {
                ValidIter::Valid(self.clone())
            } else {
                ValidIter::Done
            }
        } else {
            let index_of_unknown = self.tiles.iter().position(|t| *t == Tile::Unknown).unwrap();
            let self_ref = self;

            let iter = [Tile::Open, Tile::Spring]
                .into_iter()
                .flat_map(move |tile| {
                    self_ref.tiles[index_of_unknown] = tile;
                    let possibs = self_ref.valid_iter().collect::<Vec<_>>();
                    self_ref.tiles[index_of_unknown] = Tile::Unknown;
                    possibs
                });

            ValidIter::Iterator(Box::new(iter))
        }
    }

    pub fn valid_iter_count(&mut self) -> usize {
        if let Some(valid) = self.is_valid() {
            if valid {
                1
            } else {
                0
            }
        } else {
            let index_of_unknown = self.tiles.iter().position(|t| *t == Tile::Unknown).unwrap();
            let self_ref = self;

            let count: usize = [Tile::Open, Tile::Spring]
                .into_iter()
                .map(move |tile| {
                    self_ref.tiles[index_of_unknown] = tile;
                    if self_ref.is_partially_invalid() {
                        println!("{self_ref:?} is partially invalid");
                        self_ref.tiles[index_of_unknown] = Tile::Unknown;
                        return 0;
                    }
                    let possibs = self_ref.valid_iter_count();
                    self_ref.tiles[index_of_unknown] = Tile::Unknown;
                    possibs
                })
                .sum();

            count
        }
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in &self.tiles {
            write!(f, "{}", char::from(*tile))?;
        }
        write!(f, " ")?;
        for (idx, group_size) in self.groups.iter().copied().enumerate() {
            write!(f, "{}", group_size)?;
            if idx < self.groups.len() - 1 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ValidIter<T> {
    Valid(Row),
    Iterator(Box<T>),
    Done,
}

impl<T> Iterator for ValidIter<T>
where T: Iterator<Item = Row> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValidIter::Valid(row) => {
                let row = row.clone();
                *self = Self::Done;
                Some(row)
            },
            ValidIter::Done => None,
            ValidIter::Iterator(iter) => iter.next(),
        }
    }
}
