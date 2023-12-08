#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cycle {
    pub offset: usize,
    pub length: usize,
}
