/// The `(x, y)` coordinates of the location of a [`Tile`][`crate::tile::Tile`]. (Row major order)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    /// The location 1 unit above this one.
    pub fn up(&self) -> Self {
        let Self { x, y } = *self;
        Self { x, y: y - 1 }
    }

    /// The location 1 unit below this one.
    pub fn down(&self) -> Self {
        let Self { x, y } = *self;
        Self { x, y: y + 1 }
    }

    /// The location 1 unit to the left of this one.
    pub fn left(&self) -> Self {
        let Self { x, y } = *self;
        Self { x: x - 1, y }
    }

    /// The location 1 unit to the right of this one.
    pub fn right(&self) -> Self {
        let Self { x, y } = *self;
        Self { x: x + 1, y }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


/// A direction from one location to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    /// Get the opposite direction.
    /// 
    /// - `Down <-> Up`
    /// - `Left <-> Right`
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// The direction **TO** `this` **FROM** `relative_to`.
    pub fn from_locations(this: Location, relative_to: Location) -> Self {
        match (this.x.cmp(&relative_to.x), this.y.cmp(&relative_to.y)) {
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Self::Up,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Self::Down,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Self::Left,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Self::Right,
            _ => panic!("Locations are not adjacent"),
        }
    }
}
