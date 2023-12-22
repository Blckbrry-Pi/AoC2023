use std::{collections::{HashMap, HashSet}, fmt::Debug, sync::Arc};

use crate::brick::Brick;

#[derive(Clone, PartialEq, Eq)]
pub struct BrickLayout {
    bricks: Vec<Brick>,
}

impl BrickLayout {
    pub fn parse(input: &str) -> Self {
        let mut bricks: Vec<_> = input.lines().flat_map(Brick::parse).collect();

        bricks.sort_by_key(|brick| brick.z());
        
        let mut output = Self { bricks };
        output.fall();
        output.bricks.sort_by_key(|brick| brick.z());
        output
    }

    pub fn fall(&mut self) {
        loop {
            let mut can_shift_down = vec![true; self.bricks.len()];
            for (idx, &brick) in self.bricks.iter().enumerate() {
                if brick.z() == 0 {
                    can_shift_down[idx] = false;
                    continue;
                }
                for &other in self.bricks.iter().take(idx) {
                    if brick.intersects_horizontally(other) && brick.down().z_intersects(other) {
                        can_shift_down[idx] = false;
                    }
                }
            }

            let changed = can_shift_down.iter().any(|boolean| *boolean);

            for (shift, brick) in can_shift_down.into_iter().zip(self.bricks.iter_mut()) {
                if shift {
                    brick.set_z(brick.z() - 1);
                }
            }

            if !changed { break }
        }
        self.bricks.sort_by_key(|brick| brick.z());
    }

    pub fn build_reliance_table(&self) -> Vec<HashSet<usize>> {
        let mut reliances: Vec<_> = std::iter::repeat_with(HashSet::new).take(self.bricks.len()).collect();


        for (idx, &brick) in self.bricks.iter().enumerate() {
            if brick.z() == 0 { continue }
            for (other_idx, &other) in self.bricks.iter().enumerate() {
                if idx == other_idx { continue }
                if brick.intersects_horizontally(other) && other.contains_z(brick.z() - 1) {
                    reliances[idx].insert(other_idx);
                }
            }
        }

        reliances
    }

    pub fn disintegratable(reliance_table: &[HashSet<usize>]) -> HashSet<usize> {
        let mut disintegratable: HashSet<_>  = (0..reliance_table.len()).collect();

        for set in reliance_table {
            if set.len() == 1 {
                disintegratable.remove(set.iter().next().unwrap());
            }
        }

        disintegratable
    }

    pub fn not_disintegratable(reliance_table: &[HashSet<usize>]) -> HashSet<usize> {
        let disintegratable = Self::disintegratable(reliance_table);

        (0..reliance_table.len()).filter(|idx| !disintegratable.contains(idx)).collect()
    }

    pub fn sole_reliances(&self) -> HashMap<usize, HashSet<usize>> {
        let mut reliers = HashMap::new();
        let reliance_table = self.build_reliance_table();
        let not_disintegratable = Self::not_disintegratable(&reliance_table);

        for idx in 0..self.bricks.len() {
            if !not_disintegratable.contains(&idx) { continue }
            
            let mut fallen: HashSet<_> = [idx].into_iter().collect();
            for other_idx in idx+1..self.bricks.len() {
                if !reliance_table[other_idx].is_empty() && reliance_table[other_idx].is_subset(&fallen) {
                    fallen.insert(other_idx);
                }
            }

            fallen.remove(&idx);
            reliers.insert(idx, fallen);
        }

        reliers
    }

    pub fn is_disintegratable(&self, brick_idx: usize) -> bool {
        let mut test_self_no_fall = self.clone();
        let mut test_self_fall = self.clone();

        test_self_no_fall.bricks.remove(brick_idx);
        test_self_fall.bricks.remove(brick_idx);

        test_self_no_fall.fall();

        test_self_no_fall == test_self_fall
    }
}

impl Debug for BrickLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let max_z = self.bricks.iter().map(|b| b.z() + b.l()).max().unwrap_or_default();

        let max_horiz_idx = self.bricks
            .iter()
            .map(|b| if f.alternate() { b.y() + b.h() } else { b.x() + b.w() })
            .max()
            .unwrap_or_default();
        
        for z in (0..max_z).rev() {
            let mut line = vec!['.'; max_horiz_idx];

            for (idx, brick) in self.bricks.iter().enumerate() {
                for horiz_idx in 0..max_horiz_idx {
                    let test_brick = if f.alternate() {
                        Brick::new((0, horiz_idx, z), (10000000, horiz_idx, z))
                    } else {
                        Brick::new((horiz_idx, 0, z), (horiz_idx, 10000000, z))
                    };
                    
                    if brick.intersects_horizontally(test_brick) && brick.z_intersects(test_brick) {
                        line[horiz_idx] = ALPHABET.chars().nth(idx).unwrap_or('*');
                    }
                }
            }

            writeln!(f, "{}", line.into_iter().collect::<String>())?;
        }

        Ok(())
    }
}
