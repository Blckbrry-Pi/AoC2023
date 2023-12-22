use std::collections::HashSet;

use crate::pos::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Superchunk {
    super_pos: Pos,
    dim: isize,
}

impl Superchunk {
    pub fn new(location: Pos, dimension: isize) -> Self {
        Self { super_pos: Pos::new(location.x / dimension, location.y / dimension), dim: dimension }
    }

    pub fn new_raw(super_pos: Pos, dimension: isize) -> Self {
        Self { super_pos, dim: dimension }
    }

    pub fn super_pos(&self) -> Pos { self.super_pos }
    pub fn dim(&self) -> isize { self.dim }

    pub fn pos(&self) -> Pos { Pos::new(self.super_pos.x * self.dim, self.super_pos.y * self.dim) }

    pub fn includes(&self, location: Pos) -> bool {
        let super_pos = Pos::new(location.x / self.dim, location.y / self.dim);
        super_pos == self.super_pos
    }
    pub fn intersect_superchunk(&self, other: &Self) -> bool {
        match self.dim.cmp(&other.dim) {
            std::cmp::Ordering::Less => other.intersect_superchunk(self),
            std::cmp::Ordering::Equal => self.super_pos == other.super_pos,
            std::cmp::Ordering::Greater => other.with_dimension(self.dim).super_pos == self.super_pos,
        }
    }


    pub fn quarters(&self) -> impl Iterator<Item = Self> {
        [
            self.with_dimension(self.dim / 2),
            self.with_dimension(self.dim / 2).right(),
            self.with_dimension(self.dim / 2).down(),
            self.with_dimension(self.dim / 2).right().down(),
        ].into_iter()
    }
    
    pub fn with_dimension(&self, dimension: isize) -> Self {
        Self::new(self.pos(), dimension)
    }

    pub fn adjacent_of_size(&self, size: isize) -> HashSet<Self> {
        match self.dim.cmp(&size) {
            std::cmp::Ordering::Greater => {
                let mut adjacent = HashSet::new();

                let base_left = self.with_dimension(size).left();
                let base_up = self.with_dimension(size).up();
                let base_right = self.right().with_dimension(size);
                let base_down = self.down().with_dimension(size);
                for i in 0..(self.dim / size) {
                    adjacent.insert(Self { super_pos: base_left.super_pos + Pos::new(0, i), dim: size });
                    adjacent.insert(Self { super_pos: base_up.super_pos + Pos::new(i, 0), dim: size });
                    adjacent.insert(Self { super_pos: base_right.super_pos + Pos::new(0, i), dim: size });
                    adjacent.insert(Self { super_pos: base_down.super_pos + Pos::new(i, 0), dim: size });
                }

                adjacent
            },
            std::cmp::Ordering::Equal => {
                let mut adjacent = HashSet::new();
                adjacent.insert(self.left());
                adjacent.insert(self.up());
                adjacent.insert(self.right());
                adjacent.insert(self.down());

                adjacent
            },
            std::cmp::Ordering::Less => {
                let mut adjacent = HashSet::new();
                adjacent.insert(self.left().with_dimension(size));
                adjacent.insert(self.up().with_dimension(size));
                adjacent.insert(self.right().with_dimension(size));
                adjacent.insert(self.down().with_dimension(size));

                adjacent.remove(&self.with_dimension(size));

                adjacent
            },
        }
    }

    pub fn left(&self) -> Self { Self { super_pos: self.super_pos.left(), dim: self.dim } }
    pub fn right(&self) -> Self { Self { super_pos: self.super_pos.right(), dim: self.dim } }
    pub fn up(&self) -> Self { Self { super_pos: self.super_pos.up(), dim: self.dim } }
    pub fn down(&self) -> Self { Self { super_pos: self.super_pos.down(), dim: self.dim } }

    pub fn individuals(self) -> impl Iterator<Item = Pos> {
        (0..self.dim).flat_map(move |y| (0..self.dim).map(move |x| Pos::new(self.pos().x + x, self.pos().y + y)))
    }
}
