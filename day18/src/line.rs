use std::fmt::Debug;

use crate::{dir::Direction, color::Color};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Line {
    direction: Direction,
    length: usize,
    color: Color,
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} ({:?})", self.direction, self.length, self.color)
    }
}

impl Line {
    pub fn parse(line: &str) -> Self {
        let (dir, line) = line.split_once(' ').unwrap();
        let (len, line) = line.split_once(' ').unwrap();
        let color = line.trim_matches(['(', ')']);

        let (r, g, b) = (&color[1..3], &color[3..5], &color[5..7]);
        let color = Color::new(
            u8::from_str_radix(r, 16).unwrap(),
            u8::from_str_radix(g, 16).unwrap(),
            u8::from_str_radix(b, 16).unwrap(),
        );

        let direction = match dir {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!("Invalid direction: {}", dir),
        };

        let length = len.parse().unwrap();

        Self {
            direction,
            length,
            color,
        }
    }

    pub fn to_part_2(self) -> Self {
        let direction = match self.color.b() % 16 {
            0 => Direction::R,
            1 => Direction::D,
            2 => Direction::L,
            3 => Direction::U,
            _ => panic!("Invalid direction: {:?}", self.color),
        };

        let distance = self.color.r() as usize * 256 * 16 + self.color.g() as usize * 16 + self.color.b() as usize / 16;

        Self {
            direction,
            length: distance,
            color: Color::new(0, 0, 0),
        }
    }

    pub fn direction(&self) -> Direction { self.direction }
    pub fn length(&self) -> usize { self.length }
    pub fn color(&self) -> Color { self.color }
}