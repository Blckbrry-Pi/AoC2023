use crate::{group::GroupPos, tile::Tile, spring_row::Row};

#[derive(Debug, Clone)]
pub struct GroupManager {
    groups: Vec<GroupPos>,
}

impl GroupManager {

    pub fn from_row(row: &Row) -> Self {
        let mut manager = Self::new(row.group_positions());
        manager.filter(&row.tiles);
        manager
    }

    pub fn new(groups: Vec<GroupPos>) -> Self {
        Self { groups }
    }

    pub fn filter(&mut self, row: &[Tile]) {
        self.groups.iter_mut().for_each(|group| group.filter(row));
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    fn possibility_iter(start: usize, groups_left: &[GroupPos]) -> Box<dyn Iterator<Item = Vec<usize>> + '_> {
        if groups_left.is_empty() {
            return Box::new(std::iter::once(vec![]));
        }

        let group_0_remaining_iter = groups_left[0].iter_group_starting_at(start);

        let sub_iter = group_0_remaining_iter.flat_map(|range| {
            let new_start = range.end + 1;
            let subgroup_parts = Self::possibility_iter(new_start, &groups_left[1..]);

            subgroup_parts.map(move |mut subgroup| {
                subgroup.insert(0, range.start);
                subgroup
            })
        });

        Box::new(sub_iter)
    }

    pub fn possibilities<'a>(&'a self, row: &'a Row) -> impl Iterator<Item = Vec<usize>> + 'a {
        Self::possibility_iter(0, &self.groups)
            .filter(|possibility| {
                let mut outliers_found = false;
                let mut next_block = 0;
                let mut curr_tile_idx = 0;
                while curr_tile_idx < row.tiles.len() {
                    if next_block < possibility.len() && curr_tile_idx == possibility[next_block] {
                        curr_tile_idx += row.groups[next_block];
                        next_block += 1;
                        continue;
                    } else if row.tiles[curr_tile_idx] == Tile::Spring {
                        outliers_found = true;

                        break;
                    }
                    curr_tile_idx += 1;
                }
                !outliers_found
            })
    }
}

