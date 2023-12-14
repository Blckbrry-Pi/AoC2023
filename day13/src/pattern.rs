use std::fmt::Debug;

#[derive(Clone)]
pub struct Pattern {
    pub rows: Vec<Vec<bool>>,
}

impl Pattern {
    pub fn parse(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { rows }
    }

    pub fn check_column_axis(&self, x: usize, smudges: usize) -> bool {
        let cols_to_check = (0..x).rev().zip(x..self.rows[0].len());
        let mut smudge_count = 0;
        for (left, right) in cols_to_check {
            for row in &self.rows {
                if row[left] != row[right] {
                    smudge_count += 1;
                }
            }
        }
        smudge_count == smudges
    }

    pub fn check_row_axis(&self, y: usize, smudges: usize) -> bool {
        let rows_to_check = (0..y).rev().zip(y..self.rows.len());
        let mut smudge_count = 0;
        for (top, bottom) in rows_to_check {
            for x in 0..self.rows[0].len() {
                if self.rows[top][x] != self.rows[bottom][x] {
                    smudge_count += 1;
                }
            }
        }
        smudge_count == smudges
    }

    pub fn get_reflection_n_smudges(&self, smudges: usize) -> Axis {
        for i in 1..self.rows.len() {
            if self.check_row_axis(i, smudges) {
                return Axis::Row(i);
            }
        }

        for i in 1..self.rows[0].len() {
            if self.check_column_axis(i, smudges) {
                return Axis::Column(i);
            }
        }

        panic!("No reflection found");
    }

    pub fn get_reflection_clear(&self) -> Axis {
        self.get_reflection_n_smudges(0)
    }

    pub fn get_reflection_1_smudge(&self) -> Axis {
        self.get_reflection_n_smudges(1)
    }
}


impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for cell in row {
                write!(f, "{}", if *cell { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Row(usize),
    Column(usize),
}

impl Axis {
    pub fn value(self) -> usize {
        match self {
            Self::Row(v) => 100 * v,
            Self::Column(v) => v,
        }
    }
}
