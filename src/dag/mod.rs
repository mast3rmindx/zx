use std::collections::{HashMap, HashSet};
use crate::{Edge, Node, NodeId, Result, Error};

/// The core DAG data structure
pub struct DAG {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<NodeId, HashMap<NodeId, Edge>>,
}

impl DAG {
    /// Create a new empty DAG
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Add a node to the DAG
    pub fn add_node(&mut self, node: Node) -> Result<()> {
        self.nodes.insert(node.id.clone(), node);
        self.edges.entry(node.id).or_default();
        Ok(())
    }

    /// Add an edge to the DAG, checking for cycles
    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        // Verify nodes exist
        if !self.nodes.contains_key(&edge.from) {
            return Err(Error::NodeNotFound(edge.from));
        }
        if !self.nodes.contains_key(&edge.to) {
            return Err(Error::NodeNotFound(edge.to));
        }

        // Check for cycles before adding the edge
        if self.would_create_cycle(&edge.from, &edge.to) {
            return Err(Error::CycleDetected {
                from: edge.from.clone(),
                to: edge.to.clone(),
            });
        }

        // Add the edge
        self.edges
            .entry(edge.from.clone())
            .or_default()
            .insert(edge.to.clone(), edge);

        Ok(())
    }

    /// Get a node by its ID
    pub fn get_node(&self, id: &NodeId) -> Result<&Node> {
        self.nodes
            .get(id)
            .ok_or_else(|| Error::NodeNotFound(id.clone()))
    }

    /// Get an edge between two nodes
    pub fn get_edge(&self, from: &NodeId, to: &NodeId) -> Result<&Edge> {
        self.edges
            .get(from)
            .and_then(|edges| edges.get(to))
            .ok_or_else(|| Error::EdgeNotFound {
                from: from.clone(),
                to: to.clone(),
            })
    }

    /// Get all nodes in the DAG
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Get all edges in the DAG
    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values().flat_map(|edges| edges.values())
    }

    /// Get all outgoing edges from a node
    pub fn outgoing_edges(&self, node_id: &NodeId) -> Result<impl Iterator<Item = &Edge>> {
        if !self.nodes.contains_key(node_id) {
            return Err(Error::NodeNotFound(node_id.clone()));
        }
        Ok(self.edges
            .get(node_id)
            .map(|edges| edges.values())
            .unwrap_or_default())
    }

    /// Check if adding an edge would create a cycle
    fn would_create_cycle(&self, from: &NodeId, to: &NodeId) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![to];

        while let Some(current) = stack.pop() {
            if current == from {
                return true;
            }

            if visited.insert(current) {
                if let Some(edges) = self.edges.get(current) {
                    stack.extend(edges.keys());
                }
            }
        }

        false
    }

    /// Find all paths between two nodes
    pub fn find_paths(&self, from: &NodeId, to: &NodeId) -> Result<Vec<Vec<NodeId>>> {
        if !self.nodes.contains_key(from) {
            return Err(Error::NodeNotFound(from.clone()));
        }
        if !self.nodes.contains_key(to) {
            return Err(Error::NodeNotFound(to.clone()));
        }

        let mut paths = Vec::new();
        let mut current_path = vec![from.clone()];
        let mut visited = HashSet::new();
        visited.insert(from);

        self.find_paths_recursive(from, to, &mut visited, &mut current_path, &mut paths);

        Ok(paths)
    }

    fn find_paths_recursive(
        &self,
        current: &NodeId,
        target: &NodeId,
        visited: &mut HashSet<&NodeId>,
        current_path: &mut Vec<NodeId>,
        paths: &mut Vec<Vec<NodeId>>,
    ) {
        if current == target {
            paths.push(current_path.clone());
            return;
        }

        if let Some(edges) = self.edges.get(current) {
            for next in edges.keys() {
                if visited.insert(next) {
                    current_path.push(next.clone());
                    self.find_paths_recursive(next, target, visited, current_path, paths);
                    current_path.pop();
                    visited.remove(next);
                }
            }
        }
    }
} 