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

    fn possibility_iter<'a>(start: usize, groups_left: &'a [GroupPos], row: &'a Row, create_vecs: bool) -> Box<dyn Iterator<Item = Vec<usize>> + 'a> {
        if groups_left.is_empty() {
            if start < row.tiles.len() && row.tiles[start..].iter().any(|&tile| tile == Tile::Spring) {
                return Box::new(std::iter::empty());
            } else {
                return Box::new(std::iter::once(vec![]));
            }
        }

        let group_0_remaining_iter = groups_left[0].iter_group_starting_at(start);

        let sub_iter = group_0_remaining_iter.map_while(move |range| {
            if row.tiles[start..range.start].iter().any(|&tile| tile == Tile::Spring) {
                return None;
            }
            let new_start = range.end + 1;
            let subgroup_parts = Self::possibility_iter(new_start, &groups_left[1..], row, create_vecs);

            Some(subgroup_parts.map(move |mut subgroup| {
                if create_vecs {
                    subgroup.insert(0, range.start);
                }
                subgroup
            }))
        })
        .flatten();

        Box::new(sub_iter)
    }

    fn possibility_count_iter(start: usize, groups_left: &[GroupPos], row: &Row) -> usize {
        if groups_left.is_empty() {
            if start < row.tiles.len() && row.tiles[start..].iter().any(|&tile| tile == Tile::Spring) {
                return 0;
            } else {
                return 1;
            }
        }

        let group_0_remaining_iter = groups_left[0].iter_group_starting_at(start);

        group_0_remaining_iter.map_while(move |range| {
            if row.tiles[start..range.start].iter().any(|&tile| tile == Tile::Spring) {
                return None;
            }
            let new_start = range.end + 1;
            Some(Self::possibility_count_iter(new_start, &groups_left[1..], row))
        })
        .sum()
    }

    pub fn possibilities<'a>(&'a self, row: &'a Row, create_vecs: bool) -> impl Iterator<Item = Vec<usize>> + 'a {
        Self::possibility_iter(0, &self.groups, row, create_vecs)
    }
    pub fn possibility_count(&self, row: &Row) -> usize {
        Self::possibility_count_iter(0, &self.groups, row)
    }
}

