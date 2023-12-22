use std::{collections::HashSet, fmt::Debug};

use crate::{superchunk::Superchunk, map::Map, pos::Pos};

#[derive(Clone)]
pub struct ChunkSet {
    clear_chunks: HashSet<Superchunk>,
    nonclear_chunks: HashSet<Superchunk>,
}


impl ChunkSet {
    pub fn from_map(map: &Map, top_pow: usize) -> Self {
        let superchunk_size = 2_isize.pow(top_pow as u32);

        let mut clear_chunks = HashSet::new();
        let mut nonclear_chunks = HashSet::new();
        let x_range = (
            map.taken_tiles().map(|pos| pos.x).min().unwrap() / superchunk_size - 2,
            map.taken_tiles().map(|pos| pos.x).max().unwrap() / superchunk_size + 2,
        );
        let y_range = (
            map.taken_tiles().map(|pos| pos.y).min().unwrap() / superchunk_size - 2,
            map.taken_tiles().map(|pos| pos.y).max().unwrap() / superchunk_size + 2,
        );

        let superchunked_map = map.superchunked(superchunk_size);

        for x in x_range.0..x_range.1 {
            for y in y_range.0..y_range.1 {
                let super_pos = Pos::new(x, y);
                let superchunk = Superchunk::new_raw(super_pos, superchunk_size);
                if superchunked_map.get(superchunk.super_pos()).is_none() {
                    clear_chunks.insert(superchunk);
                } else {
                    nonclear_chunks.insert(superchunk);
                }
            }
        }

        Self { clear_chunks, nonclear_chunks }
    }

    pub fn quarter_nonclear(&self, map: &Map) -> Self {
        let mut clear_chunks = self.clear_chunks.clone();
        let mut nonclear_chunks = HashSet::new();

        let superchunked_map = map.superchunked(self.nonclear_chunks.iter().next().unwrap().dim() / 2);

        for chunk in self.nonclear_chunks.iter() {
            for quarter in chunk.quarters() {
                if superchunked_map.get(quarter.super_pos()).is_none() {
                    clear_chunks.insert(quarter);
                } else {
                    nonclear_chunks.insert(quarter);
                }
            }
        }

        Self { clear_chunks, nonclear_chunks }
    }

    pub fn full_from_map(map: &Map, top_pow: usize) -> Self {
        let mut set = Self::from_map(map, top_pow);

        while set.nonclear_chunks.iter().next().unwrap().dim() > 1 {
            set = set.quarter_nonclear(map);
            println!("{}", set.nonclear_chunks.iter().next().unwrap().dim());
        }
        
        set
    }

    fn top_leftmost_clear(&self) -> Superchunk {
        self.clear_chunks.iter()
            .copied()
            .min_by(|a, b| a.pos().x.cmp(&b.pos().x).then(a.pos().y.cmp(&b.pos().y)))
            .unwrap()
    }
    fn top_rightmost_clear(&self) -> Superchunk {
        self.clear_chunks.iter()
            .copied()
            .min_by(|a, b| b.pos().x.cmp(&a.pos().x).then(a.pos().y.cmp(&b.pos().y)))
            .unwrap()
    }
    fn bottom_leftmost_clear(&self) -> Superchunk {
        self.clear_chunks.iter()
            .copied()
            .min_by(|a, b| a.pos().x.cmp(&b.pos().x).then(b.pos().y.cmp(&a.pos().y)))
            .unwrap()
    }
    fn bottom_rightmost_clear(&self) -> Superchunk {
        self.clear_chunks.iter()
            .copied()
            .min_by(|a, b| b.pos().x.cmp(&a.pos().x).then(b.pos().y.cmp(&a.pos().y)))
            .unwrap()
    }

    pub fn get_external(&self) -> HashSet<Superchunk> {
        let max_dim = self.clear_chunks.iter().map(|c| c.dim()).max().unwrap();
        let mut external: HashSet<Superchunk> = [
            self.top_leftmost_clear(),
            self.top_rightmost_clear(),
            self.bottom_leftmost_clear(),
            self.bottom_rightmost_clear(),
        ].into_iter().collect();
        let mut to_explore = external.clone();

        loop {
            let mut new_explore = HashSet::new();

            for &chunk in to_explore.iter() {
                let mut seen_of_same_size = 0;
                for i in (0..=(max_dim as f64).log2() as u32).rev() {
                    let adjacent = chunk.adjacent_of_size(2_isize.pow(i));
                    for &new_external_chunk in self.clear_chunks.intersection(&adjacent) {
                        if !external.contains(&new_external_chunk) {
                            external.insert(new_external_chunk);
                            new_explore.insert(new_external_chunk);
                        }
                        if new_external_chunk.dim() >= chunk.dim() {
                            seen_of_same_size += 1;
                        }
                        if seen_of_same_size >= 4 {
                            break;
                        }
                    }
                    if seen_of_same_size >= 4 {
                        break;
                    }
                }
            }

            if new_explore.is_empty() { break; }

            to_explore = new_explore;

            println!("{}", to_explore.len());
        }



        external
    }

    pub fn get_internal(&self, map: &Map) -> HashSet<Superchunk> {
        let max_dim = self.clear_chunks.iter().map(|c| c.dim()).max().unwrap();
        let x_range = (
            map.taken_tiles().map(|pos| pos.x).min().unwrap() - 1,
            map.taken_tiles().map(|pos| pos.x).max().unwrap() + max_dim + 1,
        );
        let y_range = (
            map.taken_tiles().map(|pos| pos.y).min().unwrap() - 1,
            map.taken_tiles().map(|pos| pos.y).max().unwrap() + max_dim + 1,
        );
        let external = self.get_external();

        self.clear_chunks.difference(&external)
            .copied()
            .filter(|chunk| {
                let real_pos = chunk.pos();
                let x_in_range = (x_range.0..x_range.1).contains(&real_pos.x);
                let y_in_range = (y_range.0..y_range.1).contains(&real_pos.y);
                x_in_range && y_in_range
            })
            .collect()
    }

    pub fn internal_area(&self, map: &Map) -> usize {
        self.get_internal(map)
            .into_iter()
            .map(|chunk| chunk.dim() as usize * chunk.dim() as usize)
            .sum()
    }


    pub fn nonclear_area(&self) -> usize {
        let mut area = 0;
        for &chunk in self.nonclear_chunks.iter() {
            area += chunk.dim() as usize * chunk.dim() as usize;
        }
        area
    }

}

impl Debug for ChunkSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let (min, max) = Pos::iter_range(self.clear_chunks.iter().map(|chunk| chunk.super_pos()));
        let (min_x, max_x) = (min.x, max.x);
        let (min_y, max_y) = (min.y, max.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 0 && y == 0 {
                    s.push('O');
                    continue
                }
                let super_pos = Pos::new(x, y);
                for i in 0..10 {
                    if self.nonclear_chunks.contains(&Superchunk::new(super_pos, 2_isize.pow(i))) {
                        s.push(' ');
                        break
                    }
                    if i == 9 {
                        s.push('█');
                    }
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
