use std::collections::BTreeSet;
use std::fmt::Debug;

use crate::id::Id;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Node {
    id: Id,
    edges_from: BTreeSet<Id>,
    directed: bool,
}

impl Node {
    pub fn new(id: Id, directed: bool) -> Self {
        Self { id, edges_from: BTreeSet::new(), directed }
    }
    pub fn new_with_edges(id: Id, edges: impl Iterator<Item = Id>, directed: bool) -> Self {
        Self { id, edges_from: edges.collect(), directed }
    }

    pub fn id(&self) -> Id { self.id }
    pub fn directed(&self) -> bool { self.directed }

    pub fn add_edge(&mut self, edge_id: Id) {
        self.edges_from.insert(edge_id);
    }
    pub fn remove_edge(&mut self, edge_id: Id) {
        self.edges_from.remove(&edge_id);
    }

    pub fn edges_from(&self) -> impl Iterator<Item = Id> + '_ {
        self.edges_from.iter().copied()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.directed {
            write!(f, "Directed ")?;
        }
        write!(f, "Node {:?} (with outgoing edges ", self.id)?;
        for (idx, edge_id) in self.edges_from.iter().enumerate() {
            if idx == 0 {
                write!(f, "{:?}", edge_id)?;
            } else {
                write!(f, ", {:?}", edge_id)?;
            }
        }
        write!(f, ")")
    }
}