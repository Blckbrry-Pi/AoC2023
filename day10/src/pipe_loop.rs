use std::collections::HashMap;
use std::fmt::Debug;

use crate::location::Location;
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

        if start_seen.len() >= 2 {
            let location_of_start = start_seen[0];

            let mut connected_pipes = Tile::StartingPosition.get_connections(location_of_start)
                .into_iter()
                .filter(|loc| {
                    if !parts.contains_key(loc) {
                        return false;
                    }
                    for connection in dbg!(parts[loc]).get_connections(*loc) {
                        if connection == location_of_start {
                            return true;
                        }
                    }
                    false
                })
                .collect::<Vec<_>>();

            connected_pipes.sort();

            use Tile::*;
            use crate::tile::CornerType::*;

            let conn_a_above = connected_pipes[0].y < location_of_start.y;

            let conn_a_left = connected_pipes[0].x < location_of_start.x;
            let conn_b_left = connected_pipes[1].x < location_of_start.x;

            let conn_b_right = connected_pipes[1].x > location_of_start.x;

            let conn_b_below = connected_pipes[1].y > location_of_start.y;

            let starting_pipe_type = match (conn_a_above, conn_a_left, conn_b_left, conn_b_right, conn_b_below) {
                // Has above connection
                (true,      _, true,       _,       _) => Corner(TopLeft),
                (true,      _,    _,    true,       _) => Corner(TopRight),
                (true,      _,    _,       _,    true) => Vertical,

                // Has left connection
                (_,      true,    _,    true,       _) => Horizontal,
                (_,      true,    _,       _,    true) => Corner(BottomLeft),

                // Has right connection
                _ => Corner(BottomRight),
            };

            parts.insert(location_of_start, starting_pipe_type);

            Some(Self {
                starting_location: location_of_start,
                parts,
            })
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
