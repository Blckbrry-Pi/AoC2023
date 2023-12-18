use std::collections::{HashMap, HashSet};

use crate::state::State;

#[derive(Debug, Clone)]
pub struct PathSet {
    seen: HashMap<State, usize>,
    to_visit: HashSet<State>,
}

impl PathSet {
    pub fn new(starting_state: State) -> Self {
        Self {
            seen: [(starting_state, 0)].into_iter().collect(),
            to_visit: [starting_state].into_iter().collect(),
        }
    }

    pub fn do_step(&mut self, map: &crate::map::Map) -> Option<usize> {
        use std::collections::hash_map::Entry::*;

        let curr_state = *self.to_visit.iter().next()?;
        self.to_visit.remove(&curr_state);
        let curr_value = *self.seen.get(&curr_state).unwrap();

        for new_state in curr_state.next(map) {
            let new_value = map.get(new_state.x(), new_state.y()).unwrap() + curr_value;

            match self.seen.entry(new_state) {
                Occupied(mut entry) => {
                    if *entry.get() <= new_value {
                        continue;
                    } else {
                        entry.insert(new_value);
                        self.to_visit.insert(new_state);
                    }
                },
                Vacant(entry) => {
                    entry.insert(new_value);
                    self.to_visit.insert(new_state);
                },
            }
        }
        Some(self.to_visit.len())
    }

    pub fn do_step_ultra(&mut self, map: &crate::map::Map) -> Option<usize> {
        use std::collections::hash_map::Entry::*;

        let curr_state = *self.to_visit.iter().next()?;
        self.to_visit.remove(&curr_state);
        let curr_value = *self.seen.get(&curr_state).unwrap();

        for new_state in curr_state.ultra_next(map) {
            let new_value = map.get(new_state.x(), new_state.y()).unwrap() + curr_value;

            match self.seen.entry(new_state) {
                Occupied(mut entry) => {
                    if *entry.get() <= new_value {
                        continue;
                    } else {
                        entry.insert(new_value);
                        self.to_visit.insert(new_state);
                    }
                },
                Vacant(entry) => {
                    entry.insert(new_value);
                    self.to_visit.insert(new_state);
                },
            }
        }
        Some(self.to_visit.len())
    }

    pub fn insert(&mut self, state: State, steps: usize) {
        self.seen.insert(state, steps);
    }

    pub fn get(&self, state: &State) -> Option<usize> {
        self.seen.get(state).copied()
    }

    pub fn get_min(&self, x: usize, y: usize) -> Option<usize> {
        self.seen
            .iter()
            .filter(|(state, _)| state.x() == x && state.y() == y)
            .map(|(_, steps)| steps)
            .min()
            .copied()
    }
    pub fn get_min_ultra(&self, x: usize, y: usize) -> Option<usize> {
        self.seen
            .iter()
            .filter(|(state, _)| state.x() == x && state.y() == y)
            .filter(|(state, _)| state.steps() >= 4)
            .map(|(_, steps)| steps)
            .min()
            .copied()
    }

    pub fn get_path(&self, end: (usize, usize)) -> Vec<(usize, usize)> {
        let mut states = vec![];
        let mut curr_state = *self.seen
            .iter()
            .filter(|(state, _)| state.x() == end.0 && state.y() == end.1)
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0;

        while curr_state.x() != 0 || curr_state.y() != 0 {
            let curr_pos = (curr_state.x(), curr_state.y());
            let curr_dir = curr_state.dir();
            let prev_pos = curr_dir.unstep(curr_pos);
            println!("{curr_pos:?} {curr_dir:?} {prev_pos:?}");

            let next_state = *self.seen
                .iter()
                // .inspect(|(a, b)| println!("{a:?}: {b:?}"))
                .filter(|(state, _)| (state.x(), state.y()) == prev_pos)
                .filter(|(state, _)| state.x() <= end.0 && state.y() <= end.1)
                .min_by(|a, b| a.1.cmp(b.1))
                .unwrap()
                .0;

            println!("{states:#?}");
            states.push(curr_pos);
            curr_state = next_state;
        }
        states.push((0, 0));

        states
    }
}