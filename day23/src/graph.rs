use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::edge::Edge;
use crate::id::Id;
use crate::node::Node;
use crate::path::Path;

#[derive(Clone)]
pub struct Graph {
    nodes: BTreeMap<Id, Node>,
    edges: BTreeMap<Id, Edge>,

    start: Id,
    end: Id,
}

impl Graph {
    pub fn parse_p2(input: &str) -> Self {
        Self::parse(&input.chars().map(|c| {
            match c {
                '^' | 'v' | '<' | '>' => '.',
                _ => c,
            }
        }).collect::<String>())
    }

    pub fn parse(input: &str) -> Self {
        let mut nodes = BTreeMap::new();
        let mut edges = BTreeMap::new();

        let mut map = BTreeMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let node = match c {
                    '.' => Node::new(Id::new_unique(), false),
                    'v' | '>' | '<' | '^' => Node::new(Id::new_unique(), true),
                    _ => continue,
                };
                map.insert((x, y), node);
            }
        }

        for (&(x, y), node) in map.iter() {
            let id = node.id();
            let mut node = node.clone();

            let up = map.get(&(x, y - 1));
            let down = map.get(&(x, y + 1));
            let left = map.get(&(x - 1, y));
            let right = map.get(&(x + 1, y));

            let is_up = !node.directed() || input.lines().nth(y).unwrap().chars().nth(x).unwrap() == '^';
            let is_down = !node.directed() || input.lines().nth(y).unwrap().chars().nth(x).unwrap() == 'v';
            let is_left = !node.directed() || input.lines().nth(y).unwrap().chars().nth(x).unwrap() == '<';
            let is_right = !node.directed() || input.lines().nth(y).unwrap().chars().nth(x).unwrap() == '>';

            if let Some(up) = up {
                let edge = Edge::new(id, up.id(), 1);
                let edge_id = Id::new_unique();
                if is_up {
                    edges.insert(edge_id, edge);
                    node.add_edge(edge_id);
                }
            }
            if let Some(down) = down {
                let edge = Edge::new(id, down.id(), 1);
                let edge_id = Id::new_unique();
                if is_down {
                    edges.insert(edge_id, edge);
                    node.add_edge(edge_id);
                }
            }
            if let Some(left) = left {
                let edge = Edge::new(id, left.id(), 1);
                let edge_id = Id::new_unique();
                if is_left {
                    edges.insert(edge_id, edge);
                    node.add_edge(edge_id);
                }
            }
            if let Some(right) = right {
                let edge = Edge::new(id, right.id(), 1);
                let edge_id = Id::new_unique();
                if is_right {
                    edges.insert(edge_id, edge);
                    node.add_edge(edge_id);
                }
            }

            nodes.insert(id, node);
        }

        let start = map.get(&(1, 0)).unwrap().id();
        let end = map.get(&(input.chars().position(|c| c == '\n').unwrap() - 2, input.lines().count() - 1)).unwrap().id();

        Self { nodes, edges, start, end }
    }

    fn reduce_node(&mut self, node: Id) -> bool {
        let Some(node) = self.nodes.get(&node) else { return false };

        if node.edges_from().count() != 2 { return false }

        let Some(edge_id_a) = node.edges_from().next() else { return false };
        let Some(edge_id_b) = node.edges_from().last() else { return false };
        let Some(&edge_a) = self.edges.get(&edge_id_a) else { return false };
        let Some(&edge_b) = self.edges.get(&edge_id_b) else { return false };

        let node_id_a = edge_a.to();
        let node_id_b = edge_b.to();
        let Some(node_a) = self.nodes.get(&node_id_a) else { return false };
        let Some(node_b) = self.nodes.get(&node_id_b) else { return false };

        if node_a.directed() || node_b.directed() { return false }

        let Some(return_edge_id_a) = node_a
            .edges_from()
            .find(|e_id| {
                self.edges.get(e_id).map(|e| e.is_opposite_of(&edge_a)) == Some(true)
            }) else { return false };
        let Some(return_edge_id_b) = node_b
            .edges_from()
            .find(|e_id| {
                self.edges.get(e_id).map(|e| e.is_opposite_of(&edge_b)) == Some(true)
            }) else { return false };

        self.nodes.remove(&node.id());

        let a_to_b = Edge::new(node_id_a, node_id_b, edge_a.len() + edge_b.len());
        let b_to_a = Edge::new(node_id_b, node_id_a, edge_a.len() + edge_b.len());

        self.nodes.get_mut(&node_id_a).unwrap().remove_edge(return_edge_id_a);
        self.nodes.get_mut(&node_id_b).unwrap().remove_edge(return_edge_id_b);

        self.edges.remove(&edge_id_a);
        self.edges.remove(&edge_id_b);
        self.edges.remove(&return_edge_id_a);
        self.edges.remove(&return_edge_id_b);

        let edge_id_a_to_b = Id::new_unique();
        let edge_id_b_to_a = Id::new_unique();

        self.nodes.get_mut(&node_id_a).unwrap().add_edge(edge_id_a_to_b);
        self.nodes.get_mut(&node_id_b).unwrap().add_edge(edge_id_b_to_a);
        self.edges.insert(edge_id_a_to_b, a_to_b);
        self.edges.insert(edge_id_b_to_a, b_to_a);
        

        true
    }

    pub fn reduce(&mut self) {
        let mut modified = true;
        while modified {
            modified = false;
            for node in self.nodes.keys().copied().collect::<Vec<_>>() {
                if self.reduce_node(node) {
                    modified = true;
                    break;
                }
            }
        }
    }

    pub fn paths(&self) -> Vec<Path> {
        let mut completed_paths = vec![];
        let mut in_progress_paths = vec![Path::starting_at(self.start)];

        while !in_progress_paths.is_empty() {
            let mut new_paths = vec![];

            for path in in_progress_paths {
                if path.is_done(self.end) {
                    completed_paths.push(path);
                    continue;
                }

                new_paths.extend(path.take_step(self));
            }

            in_progress_paths = new_paths;
        }

        completed_paths
    }

    pub fn edge(&self, id: Id) -> Option<&Edge> {
        self.edges.get(&id)
    }

    pub fn node(&self, id: Id) -> Option<&Node> {
        self.nodes.get(&id)
    }
}


impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Graph {{")?;
        writeln!(f, "  Start: {:?}", self.start)?;
        writeln!(f, "  End: {:?}", self.end)?;
        writeln!(f, "  Nodes:")?;
        for node in self.nodes.values() {
            writeln!(f, "    {:?}", node)?;
        }
        writeln!(f, "  Edges:")?;
        for (id, edge) in self.edges.iter() {
            writeln!(f, "    {id:?}: {:?}", edge)?;
        }
        writeln!(f, "}}")
    }
}
