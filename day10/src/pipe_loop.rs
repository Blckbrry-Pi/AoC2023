use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::location::{Location, Direction};
use crate::tile::{Tile, CornerType};
use crate::map::Map;

#[derive(Clone, PartialEq, Eq)]
pub struct Loop {
    starting_location: Location,
    parts: HashMap<Location, Tile>,
}

impl Loop {
    pub fn try_build(starting_at: Location, map: &Map) -> Option<Self> {
        let mut parts = HashMap::new();
        
        let mut to_check = vec![starting_at];
        let mut start_seen = vec![];

        while let Some(loc) = to_check.pop() {
            if !map.in_bounds(loc) {
                continue;
            }
            let tile = map[loc];

            if matches!(tile, Tile::StartingPosition) {
                start_seen.push(loc);
            }
            if tile == Tile::Empty {
                continue;
            }
            if parts.contains_key(&loc) {
                continue;
            }

            parts.insert(loc, tile);
            to_check.extend(tile.get_connections(loc));
        }

        println!();

        // If the loop connected to start twice, then we have a valid loop.
        if start_seen.len() >= 2 {
            let location_of_start = start_seen[0];

            // Get the locations of all the tiles that are connected to the
            // starting location. (should be 2)
            let connected_pipes = Tile::StartingPosition.get_connections(location_of_start)
                .into_iter()
                .filter(|loc| parts.contains_key(loc) && parts[loc].connects(*loc, location_of_start))
                .collect::<Vec<_>>();

            // Get the relative direction from the start pipe to the two
            // connected pipe tiles.
            let a = Direction::from_locations(connected_pipes[0], location_of_start);
            let b = Direction::from_locations(connected_pipes[1], location_of_start);

            // Turn the directions into a tile type.
            let starting_pipe_type = Tile::from_directions(a, b);

            // Overwrite the "starting position" tile with the correct pipe
            parts.insert(location_of_start, starting_pipe_type);


            Some(Self {
                starting_location: location_of_start,
                parts,
            })
        
        // If not, it either isn't a loop, or isn't the loop we're looking for
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.parts.len()
    }

    pub fn contains(&self, location: Location) -> bool {
        self.parts.contains_key(&location)
    }

    pub fn intersections_right(&self, location: Location, width: usize) -> usize {

        let rightward_iter = (location.x + 1..width).map(|x| Location { x, y: location.y });
        rightward_iter
            .filter(|&loc| {
                if !self.contains(loc) {
                    return false;
                }
                matches!(self.parts[&loc],  Tile::Vertical | Tile::Corner(CornerType::BottomLeft | CornerType::BottomRight))
            })
            .count()
    }

    pub fn inside_pipe_loop(&self, location: Location, width: usize) -> bool {
        !self.contains(location) && self.intersections_right(location, width) % 2 == 1
    }

    pub fn debug_with_map<'a>(&'a self, map: &'a Map) -> Debugger<'a> {
        Debugger(map, self)
    }
}

pub struct Debugger<'a>(&'a Map, &'a Loop);

impl Debug for Debugger<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.0.tiles.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let loc = Location { x, y };
                if self.1.contains(loc) {
                    write!(f, "{}", self.1.parts[&loc])?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Display for Debugger<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.0.tiles.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let loc = Location { x, y };
                if self.1.contains(loc) {
                    write!(f, "{:#}", self.1.parts[&loc])?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
