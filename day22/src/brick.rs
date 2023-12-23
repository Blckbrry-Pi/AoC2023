use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Brick {
    x: usize,
    y: usize,
    z: usize,

    w: usize,
    h: usize,
    l: usize,
}

impl Brick {
    pub fn new(corner_a: (usize, usize, usize), corner_b: (usize, usize, usize)) -> Self {
        Self {
            x: corner_a.0,
            y: corner_a.1,
            z: corner_a.2,

            w: corner_b.0 - corner_a.0 + 1,
            h: corner_b.1 - corner_a.1 + 1,
            l: corner_b.2 - corner_a.2 + 1,
        }
    }

    pub fn parse(line: &str) -> Option<Self> {
        let (corner_a, corner_b) = line.split_once('~')?;

        let corner_a: Result<Vec<usize>, _> = corner_a.splitn(3, ',').map(str::parse).collect();
        let corner_a = corner_a.ok()?;
        
        let corner_b: Result<Vec<usize>, _> = corner_b.splitn(3, ',').map(str::parse).collect();
        let corner_b = corner_b.ok()?;

        let corner_a = (corner_a[0], corner_a[1], corner_a[2]);
        let corner_b = (corner_b[0], corner_b[1], corner_b[2]);

        Some(Self::new(corner_a, corner_b))
    }

    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }
    pub fn z(&self) -> usize { self.z }

    pub fn set_z(&mut self, z: usize) { self.z = z; }
    pub fn down(&self) -> Self { Self { z: self.z - 1, ..*self } }

    pub fn w(&self) -> usize { self.w }
    pub fn h(&self) -> usize { self.h }
    pub fn l(&self) -> usize { self.l }

    pub fn intersects_horizontally(self, other: Self) -> bool {
        let x_intersects_variant_a = self.x <= other.x && other.x < self.x + self.w;
        let x_intersects_variant_b = other.x <= self.x && self.x < other.x + other.w;
        let x_intersects_variant_c = self.x <= other.x && other.x + other.w <= self.x + self.w;
        let x_intersects_variant_d = other.x <= self.x && self.x + self.w <= other.x + other.w;
        let x_intersects = x_intersects_variant_a || x_intersects_variant_b || x_intersects_variant_c || x_intersects_variant_d;

        let y_intersects_variant_a = self.y <= other.y && other.y < self.y + self.h;
        let y_intersects_variant_b = other.y <= self.y && self.y < other.y + other.h;
        let y_intersects_variant_c = self.y <= other.y && other.y + other.h <= self.y + self.h;
        let y_intersects_variant_d = other.y <= self.y && self.y + self.h <= other.y + other.h;
        let y_intersects = y_intersects_variant_a || y_intersects_variant_b || y_intersects_variant_c || y_intersects_variant_d;

        x_intersects && y_intersects
    }

    pub fn z_intersects(self, other: Self) -> bool {
        let z_intersects_variant_a = self.z <= other.z && other.z < self.z + self.l;
        let z_intersects_variant_b = other.z <= self.z && self.z < other.z + other.l;
        let z_intersects_variant_c = self.z <= other.z && other.z + other.l <= self.z + self.l;
        let z_intersects_variant_d = other.z <= self.z && self.z + self.l <= other.z + other.l;

        z_intersects_variant_a || z_intersects_variant_b || z_intersects_variant_c || z_intersects_variant_d
    }

    pub fn contains_z(self, z: usize) -> bool {
        self.z <= z && z < self.z + self.l
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.x, self.y, self.z,
            self.x + self.w - 1, self.y + self.h - 1, self.z + self.l - 1,
        )
    }
}
