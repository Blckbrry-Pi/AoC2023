use std::fmt::Debug;

use crate::map::Map;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pos: (usize, usize),
    direction: Direction,
    steps_taken: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction { Up, Right, Down, Left }

impl Direction {
    pub fn unstep(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0, pos.1 + 1),
            Direction::Right => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0, pos.1 - 1),
            Direction::Left => (pos.0 + 1, pos.1),
        }
    }
    pub fn step(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) {} steps {:?}", self.pos.0, self.pos.1, self.steps_taken, self.direction)
    }
}

impl State {
    pub fn new(x: usize, y: usize, direction: Direction, steps_taken: u8) -> Self {
        Self {
            pos: (x, y),
            direction,
            steps_taken
        }
    }

    pub fn next(self, map: &Map) -> impl Iterator<Item = State> + '_ {
        let directions = match self.direction {
            Direction::Up => [Direction::Left, Direction::Up, Direction::Right],
            Direction::Right => [Direction::Up, Direction::Right, Direction::Down],
            Direction::Down => [Direction::Right, Direction::Down, Direction::Left],
            Direction::Left => [Direction::Down, Direction::Left, Direction::Up],
        };

        directions
            .into_iter()
            .map(move |dir| (dir, dir.step(self.pos)))
            .filter(|(_, (x, y))| map.get(*x, *y).is_some())
            .filter(move |(dir, _)| self.direction != *dir || self.steps_taken <= 2)
            .map(move |(dir, pos)| State {
                pos,
                direction: dir,
                steps_taken: if dir == self.direction { self.steps_taken + 1 } else { 1 },
            })
    }

    pub fn ultra_next(self, map: &Map) -> impl Iterator<Item = State> + '_ {
        let directions = match self.direction {
            Direction::Up => [Direction::Left, Direction::Up, Direction::Right],
            Direction::Right => [Direction::Up, Direction::Right, Direction::Down],
            Direction::Down => [Direction::Right, Direction::Down, Direction::Left],
            Direction::Left => [Direction::Down, Direction::Left, Direction::Up],
        };

        directions
            .into_iter()
            .map(move |dir| (dir, dir.step(self.pos)))
            .filter(|(_, (x, y))| map.get(*x, *y).is_some())
            .filter(move |(dir, _)| if self.steps_taken == 0 {
                true
            } else if self.direction != *dir && self.steps_taken < 4 {
                false
            } else {
                !(self.direction == *dir && self.steps_taken >= 10)
            })
            .map(move |(dir, pos)| State {
                pos,
                direction: dir,
                steps_taken: if dir == self.direction { self.steps_taken + 1 } else { 1 },
            })
    }

    pub fn x(&self) -> usize {
        self.pos.0
    }
    pub fn y(&self) -> usize {
        self.pos.1
    }

    pub fn dir(&self) -> Direction {
        self.direction
    }

    pub fn steps(&self) -> u8 {
        self.steps_taken
    }
}
