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
