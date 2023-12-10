use crate::{node::DirectionNode, ident::Ident};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn access(self, node: &DirectionNode) -> Ident {
        match self {
            Self::Left => node.left,
            Self::Right => node.right,
        }
    }

    pub fn parse(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}




#[derive(Clone)]
pub struct Directions {
    directions: Vec<Direction>,
}

impl Directions {
    pub fn new() -> Self { Self { directions: vec![] } }

    pub fn parse(line: &str) -> Option<Self> {
        Some(Self { directions: line.chars().map(Direction::parse).collect::<Option<Vec<_>>>()? })
    }

    pub fn push(&mut self, direction: Direction) {
        self.directions.push(direction);
    }

    pub fn into_indexed_iter(self) -> IndexedDirectionsIter {
        IndexedDirectionsIter::new(self)
    }
}

impl Default for Directions {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for direction in &self.directions {
            write!(f, "{direction:?}")?;
        }
        Ok(())
    }
}

impl std::iter::IntoIterator for Directions {
    type Item = Direction;
    type IntoIter = DirectionsIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionsIter::new(self)
    }
}




pub struct DirectionsIter(IndexedDirectionsIter);

impl DirectionsIter {
    fn new(directions: Directions) -> Self {
        Self(IndexedDirectionsIter::new(directions))
    }
}

impl Iterator for DirectionsIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, direction)| direction)
    }
}




pub struct IndexedDirectionsIter {
    directions: Directions,
    curr: usize,
}

impl IndexedDirectionsIter {
    fn new(directions: Directions) -> Self {
        Self {
            directions,
            curr: 0,
        }
    }
}

impl Iterator for IndexedDirectionsIter {
    type Item = (usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.directions.directions.get(self.curr)?;
        let index = self.curr;

        self.curr += 1;
        if self.curr == self.directions.directions.len() {
            self.curr = 0;
        }

        Some((index, *direction))
    }
}
