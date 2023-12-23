use std::{collections::BTreeSet, fmt::Debug};

use crate::graph::Graph;
use crate::id::Id;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Path {
    start_node: Id,
    curr_node: Id,
    edges_taken: Vec<Id>,
    edges_set: BTreeSet<Id>,
    nodes_seen: BTreeSet<Id>,
}

impl Path {
    pub fn starting_at(node: Id) -> Self {
        Self {
            start_node: node,
            curr_node: node,
            edges_taken: Vec::new(),
            edges_set: BTreeSet::new(),
            nodes_seen: [node].into_iter().collect(),
        }
    }

    pub fn take_step(&self, graph: &Graph) -> Vec<Self> {
        let edges: Vec<_> = graph
            .node(self.curr_node).unwrap()
            .edges_from().filter(|edge_id| !self.edges_set.contains(edge_id))
            .collect();

        if edges.is_empty() { return vec![] }

        let pairs = edges.iter().filter_map(|edge_id| {
            let edge = graph.edge(*edge_id).unwrap();
            let node_id = edge.to();
            if self.nodes_seen.contains(&node_id) {
                None
            } else {
                Some((edge_id, node_id))
            }
        }).collect::<Vec<_>>();

        let mut paths = vec![];

        for (edge_id, node_id) in pairs {
            let mut path = self.clone();
            path.curr_node = node_id;
            path.edges_taken.push(*edge_id);
            path.edges_set.insert(*edge_id);
            path.nodes_seen.insert(node_id);
            paths.push(path);
        }

        paths
    }

    pub fn is_done(&self, target: Id) -> bool {
        self.curr_node == target
    }

    pub fn len(&self, map: &Graph) -> usize {
        self.edges_taken.iter().map(|edge_id| map.edge(*edge_id).unwrap().len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.edges_taken.is_empty()
    }
}



impl<'a> Debug for PathDebugger<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{:?}}}", self.path.start_node)?;
        for edge in self.path.edges_taken.iter() {
            let node_id = self.graph.edge(*edge).unwrap().to();
            write!(f, "--{{{node_id:?}}}")?;
        }
        Ok(())
    }
}

pub struct PathDebugger<'a> {
    path: &'a Path,
    graph: &'a Graph,
}
