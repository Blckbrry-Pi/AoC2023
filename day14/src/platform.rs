use std::fmt::Debug;

use crate::rock::TileType;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    tiles: Vec<Vec<TileType>>,
}

impl Platform {
    pub fn parse(input: &str) -> Self {
        Self {
            tiles: input
                .lines()
                .map(|line| line.chars().map(TileType::from_char).collect())
                .collect(),
        }
    }

    fn shift_up_once(&mut self) -> bool {
        let mut shifted = false;

        for y in 1..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                let curr_tile = self.tiles[y][x];
                let above_tile = self.tiles[y - 1][x];

                if curr_tile == TileType::RoundRock && above_tile == TileType::Empty {
                    self.tiles[y][x] = TileType::Empty;
                    self.tiles[y - 1][x] = TileType::RoundRock;
                    shifted = true;
                }
            }
        }

        shifted
    }

    pub fn shift_right_once(&mut self) -> bool {
        let mut shifted = false;

        for y in 0..self.tiles.len() {
            for x in (0..self.tiles[0].len()-1).rev() {
                let curr_tile = self.tiles[y][x];
                let right_tile = self.tiles[y][x + 1];

                if curr_tile == TileType::RoundRock && right_tile == TileType::Empty {
                    self.tiles[y][x] = TileType::Empty;
                    self.tiles[y][x + 1] = TileType::RoundRock;
                    shifted = true;
                }
            }
        }

        shifted
    }

    pub fn shift_down_once(&mut self) -> bool {
        let mut shifted = false;

        for y in (0..self.tiles.len() - 1).rev() {
            for x in 0..self.tiles[0].len() {
                let curr_tile = self.tiles[y][x];
                let below_tile = self.tiles[y + 1][x];

                if curr_tile == TileType::RoundRock && below_tile == TileType::Empty {
                    self.tiles[y][x] = TileType::Empty;
                    self.tiles[y + 1][x] = TileType::RoundRock;
                    shifted = true;
                }
            }
        }

        shifted
    }

    pub fn shift_left_once(&mut self) -> bool {
        let mut shifted = false;

        for y in 0..self.tiles.len() {
            for x in 1..self.tiles[0].len() {
                let curr_tile = self.tiles[y][x];
                let left_tile = self.tiles[y][x - 1];

                if left_tile == TileType::Empty && curr_tile == TileType::RoundRock {
                    self.tiles[y][x] = TileType::Empty;
                    self.tiles[y][x - 1] = TileType::RoundRock;
                    shifted = true;
                }
            }
        }

        shifted
    }

    pub fn shift_up(&mut self) {
        while self.shift_up_once() {}
    }

    pub fn shift_right(&mut self) {
        while self.shift_right_once() {}
    }

    pub fn shift_down(&mut self) {
        while self.shift_down_once() {}
    }

    pub fn shift_left(&mut self) {
        while self.shift_left_once() {}
    }

    pub fn spin(&mut self) {
        self.shift_up();
        self.shift_left();
        self.shift_down();
        self.shift_right();
    }

    pub fn calculate_load(self, load_fn: impl Fn(TileType, (usize, usize)) -> usize) -> usize {
        self.tiles
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, tile)| (tile, (x, y)))
            })
            .map(|(tile, pos)| load_fn(tile, pos))
            .sum()
    }

    pub fn calculate_load_north_beam(self) -> usize {
        let (_, h) = (self.width(), self.height());
        self.calculate_load(|tile_type, (_, y)| {
            if tile_type == TileType::RoundRock {
                h - y
            } else {
                0
            }
        })
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                std::fmt::Write::write_char(f, tile.to_char())?;
            }
            std::fmt::Write::write_char(f, '\n')?;
        }
        Ok(())
    }
}
