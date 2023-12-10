use crate::ident::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    ident: Ident,
    direction_index: usize,
}

impl State {
    pub fn new(ident: Ident, direction_index: usize) -> Self {
        Self {
            ident,
            direction_index,
        }
    }
}
