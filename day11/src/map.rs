use std::fmt::Debug;

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Vec<bool>>,
}

impl Map {
    pub fn parse(lines: &[&str]) -> Self {
        let tiles = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Invalid tile: {}", c),
                    })
                    .collect()
            })
            .collect();
        Self { tiles }
    }
    pub fn new(tiles: Vec<Vec<bool>>) -> Self {
        Self { tiles }
    }

    pub fn expanded_galaxies(self, factor: usize) -> Vec<(usize, usize)> {
        let extras = factor - 1;

        let empty_columns: Vec<_> = (0..self.width())
            .filter(|col_idx| self.tiles.iter().map(|row| row[*col_idx]).all(|tile| !tile))
            .collect();

        let empty_rows: Vec<_> = (0..self.height())
            .filter(|row_idx| self.tiles[*row_idx].iter().all(|tile| !tile))
            .collect();

        let mut galaxies = vec![];        

        for (y, row) in self.tiles.into_iter().enumerate() {
            for (x, tile) in row.into_iter().enumerate() {
                let rows_above = empty_rows.iter().filter(|row_idx| **row_idx < y).count() * extras;
                let columns_left = empty_columns.iter().filter(|col_idx| **col_idx < x).count() * extras;

                if tile {
                    galaxies.push((y + rows_above, x + columns_left));
                }
            }
        }

        galaxies
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.tiles.get(y).and_then(|row| row.get(x).copied())
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get(x, y).unwrap() {
                    galaxies.push((x, y));
                }
            }
        }

        galaxies
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", if *tile { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
