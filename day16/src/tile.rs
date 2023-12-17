use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Mirror(MirrorDirection),
    Splitter(SplitterDirection),
}

impl Tile {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '\\' => Tile::Mirror(MirrorDirection::SlopeDown),
            '/' => Tile::Mirror(MirrorDirection::SlopeUp),
            '-' => Tile::Splitter(SplitterDirection::Horizontal),
            '|' => Tile::Splitter(SplitterDirection::Vertical),
            _ => panic!("Invalid tile: {}", c),
        }
    }

    pub fn is_empty(&self) -> bool { matches!(self, Self::Empty) }
    pub fn is_mirror(&self) -> bool { matches!(self, Self::Mirror(_)) }
    pub fn is_splitter(&self) -> bool { matches!(self, Self::Splitter(_)) }

    pub fn as_mirror(&self) -> Option<MirrorDirection> {
        match self {
            Tile::Mirror(direction) => Some(*direction),
            _ => None,
        }
    }

    pub fn as_splitter(&self) -> Option<SplitterDirection> {
        match self {
            Tile::Splitter(direction) => Some(*direction),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Mirror(direction) => match direction {
                MirrorDirection::SlopeDown => '\\',
                MirrorDirection::SlopeUp => '/',
            },
            Tile::Splitter(direction) => match direction {
                SplitterDirection::Horizontal => '-',
                SplitterDirection::Vertical => '|',
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MirrorDirection {
    SlopeDown,
    SlopeUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplitterDirection {
    Horizontal,
    Vertical,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}