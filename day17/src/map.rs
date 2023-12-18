use std::fmt::Debug;

#[derive(Clone)]
pub struct Map {
    blocks: Vec<Vec<usize>>,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let blocks = input
            .lines()
            .map(|line| line.chars().map(|c| c as usize - b'0' as usize).collect())
            .collect();

        Self { blocks }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.blocks.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn width(&self) -> usize {
        self.blocks[0].len()
    }
    pub fn height(&self) -> usize {
        self.blocks.len()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.blocks {
            for block in row {
                write!(f, "{}", block)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
