use std::fmt::Debug;

use crate::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    from: Id,
    to: Id,

    len: usize,
}

impl Edge {
    pub fn new(from: Id, to: Id, len: usize) -> Self {
        Self { from, to, len }
    }

    pub fn is_similar_to(&self, other: &Self) -> bool {
        self == other || self.is_opposite_of(other)
    }
    pub fn is_opposite_of(&self, other: &Self) -> bool {
        self.from == other.to && self.to == other.from && self.len == other.len
    }

    pub fn reversed(&self) -> Self { Self::new(self.to, self.from, self.len) }

    pub fn from(&self) -> Id { self.from }
    pub fn to(&self) -> Id { self.to }
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize { self.len }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} --{{{len: ^5}}}-> {:?}", self.from, self.to, len = self.len)
    }
}
