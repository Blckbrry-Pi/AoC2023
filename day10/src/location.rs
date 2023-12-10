#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let y_cmp = self.y.cmp(&other.y);
        if y_cmp == std::cmp::Ordering::Equal {
            self.x.cmp(&other.x)
        } else {
            y_cmp
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Above = 0,
    Below = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Above => Self::Below,
            Self::Below => Self::Above,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn from_locations(this: Location, relative_to: Location) -> Self {
        match (this.x.cmp(&relative_to.x), this.y.cmp(&relative_to.y)) {
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Self::Above,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Self::Below,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Self::Left,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Self::Right,
            _ => panic!("Locations are not adjacent"),

        }
    }
}
