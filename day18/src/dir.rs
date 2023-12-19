use crate::pos::Pos;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction { U, D, L, R }

impl Direction {
    pub fn step(&self, pos: Pos) -> Pos {
        match self {
            Direction::U => pos.up(),
            Direction::D => pos.down(),
            Direction::L => pos.left(),
            Direction::R => pos.right(),
        }
    }
}
