use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lens<'a> {
    name: &'a str,
    focal_length: usize,
}

impl Debug for Lens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.name, self.focal_length)
    }
}

impl<'a> Lens<'a> {
    pub fn new(name: &'a str, focal_length: usize) -> Self {
        Self { name, focal_length }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn focal_length(&self) -> usize {
        self.focal_length
    }
}
