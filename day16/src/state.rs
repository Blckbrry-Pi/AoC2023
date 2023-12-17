use std::fmt::Debug;

use crate::tile::Tile;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct State {
    tiles: Vec<Vec<TileState>>,
}

impl State {
    pub fn new(map: &crate::map::Map) -> Self {
        let tiles = vec![vec![TileState { up: false, down: false, left: false, right: false }; map.width()]; map.height()];

        Self { tiles }
    }

    pub fn with_on_left(mut self, col: usize) -> Self {
        self.tiles[col][0].right = true;
        self
    }

    pub fn with_on_right(mut self, col: usize) -> Self {
        let x_idx = self.tiles[col].len() - 1;
        self.tiles[col][x_idx].left = true;
        self
    }

    pub fn with_on_top(mut self, row: usize) -> Self {
        self.tiles[0][row].down = true;
        self
    }

    pub fn with_on_bottom(mut self, row: usize) -> Self {
        let y_idx = self.tiles.len() - 1;
        self.tiles[y_idx][row].up = true;
        self
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TileState> {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn set(&mut self, x: usize, y: usize, tile: TileState) {
        self.tiles[y][x] = tile;
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for row in &self.tiles {
            for tile in row {
                if tile.up || tile.down || tile.left || tile.right {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn step(&self, map: &crate::map::Map) -> Self {
        let mut new_tiles = self.tiles.clone();

        for y in 0..map.height() {
            for x in 0..map.width() {
                let tile = map.get(x, y).unwrap();
                let state = self.get(x, y).unwrap();

                match tile {
                    Tile::Empty => {
                        if state.left && x > 0 {
                            new_tiles[y][x-1].left = true;
                        }
                        if state.right && x < map.width() - 1 {
                            new_tiles[y][x+1].right = true;
                        }
                        if state.up && y > 0 {
                            new_tiles[y-1][x].up = true;
                        }
                        if state.down && y < map.height() - 1 {
                            new_tiles[y+1][x].down = true;
                        }
                    },
                    Tile::Mirror(crate::tile::MirrorDirection::SlopeDown) => {
                        if state.right && y < map.height() - 1 {
                            new_tiles[y+1][x].down = true;
                        }
                        if state.left && y > 0 {
                            new_tiles[y-1][x].up = true;
                        }
                        if state.up && x > 0 {
                            new_tiles[y][x-1].left = true;
                        }
                        if state.down && x < map.width() - 1 {
                            new_tiles[y][x+1].right = true;
                        }
                    },
                    Tile::Mirror(crate::tile::MirrorDirection::SlopeUp) => {
                        if state.right && y > 0 {
                            new_tiles[y-1][x].up = true;
                        }
                        if state.left && y < map.height() - 1 {
                            new_tiles[y+1][x].down = true;
                        }
                        if state.up && x < map.width() - 1 {
                            new_tiles[y][x+1].right = true;
                        }
                        if state.down && x > 0 {
                            new_tiles[y][x-1].left = true;
                        }
                    },
                    Tile::Splitter(crate::tile::SplitterDirection::Horizontal) => {
                        if state.up || state.down {
                            if x > 0 {
                                new_tiles[y][x-1].left = true;
                            }
                            if x < map.width() - 1 {
                                new_tiles[y][x+1].right = true;
                            }
                        }
                        if state.left && x > 0 {
                            new_tiles[y][x-1].left = true;
                        }
                        if state.right && x < map.width() - 1 {
                            new_tiles[y][x+1].right = true;
                        }
                    },
                    Tile::Splitter(crate::tile::SplitterDirection::Vertical) => {
                        if state.left || state.right {
                            if y > 0 {
                                new_tiles[y-1][x].up = true;
                            }
                            if y < map.height() - 1 {
                                new_tiles[y+1][x].down = true;
                            }
                        }
                        if state.up && y > 0 {
                            new_tiles[y-1][x].up = true;
                        }
                        if state.down && y < map.height() - 1 {
                            new_tiles[y+1][x].down = true;
                        }
                    },
                }
            }

        }
        Self { tiles: new_tiles }
    }

    pub fn step_until_done(&self, map: &crate::map::Map) -> Self {
        let mut curr_state = self.clone();
        loop {
            let next_state = curr_state.step(map);
            if next_state == curr_state {
                break;
            }
            curr_state = next_state;
        }
        curr_state
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let has_multiple = tile.up as usize + tile.down as usize + tile.left as usize + tile.right as usize > 1;
                let tile = if has_multiple { '#' } else if tile.up { '^' } else if tile.down { 'v' } else if tile.left { '<' } else if tile.right { '>' } else { ' ' };
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
