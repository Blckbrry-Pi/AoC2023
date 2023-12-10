use crate::ident::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirectionNode {
    pub left: Ident,
    pub right: Ident,
}

impl DirectionNode {
    pub fn new(left: Ident, right: Ident) -> Self {
        Self {
            left,
            right,
        }
    }
}
