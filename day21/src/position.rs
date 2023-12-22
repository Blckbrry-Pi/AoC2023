use std::fmt::Debug;
use std::ops::Add;

use crate::step::Step;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{x}, {y}]", x = self.x, y = self.y)
    }
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }
}

impl Add<Step> for Position {
    type Output = Self;

    fn add(self, rhs: Step) -> Self::Output {
        let y_change = match rhs {
            Step::NN | Step::NE | Step::NW => -1,
            Step::SS | Step::SE | Step::SW => 1,
            Step::EE | Step::WW => 0,
        };

        let x_change = match rhs {
            Step::NN | Step::SS => 0,
            Step::NE | Step::SE | Step::EE => 1,
            Step::NW | Step::SW | Step::WW => -1,
        };

        Self::new(self.x + x_change, self.y + y_change)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
