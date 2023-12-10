use std::collections::HashMap;

use crate::ident::Ident;
use crate::node::DirectionNode;

#[derive(Debug)]
pub struct Map {
    pub nodes: HashMap<Ident, DirectionNode>,
    pub start: Ident,
    pub end: Ident,
}

impl Map {
    pub fn add_line(&mut self, line: &str) -> Option<(Ident, DirectionNode)> {
        // All of the identifiers are 3 characters, so we can just do this:
        let first_ident = Ident::new(&line[..3]);
        let l_ident = Ident::new(&line[7..10]);
        let r_ident = Ident::new(&line[12..15]);

        let direction = DirectionNode::new(l_ident, r_ident);
        self.nodes.insert(first_ident, direction);

        Some((first_ident, direction))
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            start: Ident::AAA,
            end: Ident::ZZZ,
        }
    }
}
