use std::collections::{ HashMap, HashSet };


use crate::position::Position;
use crate::map::Map;
use crate::step::Step;
use crate::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfState {
    new_positions: HashSet<(Position, Position)>,
    seen_positions: HashMap<(Position, Position), bool>,

    new_seen_blocks: HashSet<Position>,
    seen_blocks: HashMap<Position, (usize, usize)>,
    completed_blocks: HashMap<Position, (usize, usize)>,
}

impl ElfState {
    pub fn new(starting_position: Position) -> Self {
        let mut output = Self {
            new_positions: [(starting_position, Position::new(0, 0))].into_iter().collect(),
            seen_positions: HashMap::default(),

            new_seen_blocks: [Position::new(0, 0)].into_iter().collect(),
            seen_blocks: HashMap::default(),
            completed_blocks: HashMap::default(),
        };

        output.new_positions.reserve(1_000_000);
        output.seen_positions.reserve(1_000_000);
        output.completed_blocks.reserve(1_000_000);

        output
    }

    pub fn step(&mut self, map: &Map, parity: bool) {
        let to_step_from = std::mem::take(&mut self.new_positions);
        self.new_seen_blocks.clear();

        for (pos, offset) in to_step_from {
            self.seen_positions.insert((pos, offset), !parity);

            let counts = self.seen_blocks.entry(offset).or_default();
            if parity { counts.0 += 1 } else { counts.1 += 1 }

            for step in Step::orthogonal() {
                let (new_pos, offset_offset) = map.mod_position(pos + step);

                let new_offset = offset + offset_offset;

                if !matches!(map.get_pos(new_pos), Some(Tile::GardenPlot { .. })) { continue }
                if self.completed_blocks.contains_key(&new_offset) { continue }
                if self.seen_positions.contains_key(&(new_pos, new_offset)) { continue }

                self.new_positions.insert((new_pos, new_offset));
                self.new_seen_blocks.insert(new_offset);
            }
        }
    }

    pub fn reduce(&mut self, map: &Map) {
        let mut to_remove = HashSet::new();
        for offset in self.seen_blocks.keys().filter(|key| !self.new_seen_blocks.contains(key)).copied() {
            for y in 0..map.height() {
                for x in 0..map.width() {
                    let pos_to_check = Position::new(x as isize, y as isize);
                    self.seen_positions.remove(&(pos_to_check, offset));
                }
            }
            let (parity_f, parity_t) = self.seen_blocks[&offset];

            self.completed_blocks.insert(offset, (parity_f, parity_t));
            to_remove.insert(offset);
        }

        for offset in to_remove {
            self.seen_blocks.remove(&offset);
        }
    }

    pub fn count(&self, parity: bool) -> usize {
        self.count_individuals(parity) + self.count_chunks(parity)
    }
    pub fn count_chunks(&self, parity: bool) -> usize {
        self.completed_blocks
            .values()
            .map(|(parity_f, parity_t)| if parity { parity_t } else { parity_f })
            .sum()
    }
    pub fn count_individuals(&self, parity: bool) -> usize {
        self.seen_positions.iter().filter(|(_, p)| **p == parity).count() + self.new_positions.len()
    }
}


