pub mod ident;
pub mod node;
pub mod direction;
pub mod state;
pub mod cycle;

use std::collections::HashMap;

use ident::Ident;
use node::DirectionNode;



const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";


#[derive(Debug)]
pub struct Map {
    pub nodes: HashMap<Ident, DirectionNode>,
    pub start: ident::Ident,
    pub end: ident::Ident,
}

impl Map {
    pub fn add_line(&mut self, line: &str) -> Option<(Ident, DirectionNode)> {
        let first_ident = Ident::new(line.split_once(' ')?.0);

        let pair = line.split_once(" = ")?.1;

        
        let l_ident = pair.trim_start_matches('(').split_once(", ")?.0;
        let r_ident = pair.trim_end_matches(')').split_once(", ")?.1;

        let direction = DirectionNode::new(
            Ident::new(l_ident),
            Ident::new(r_ident),
        );

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
