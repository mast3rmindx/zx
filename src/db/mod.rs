use parity_db::{Db, Options, Column};
use crate::{Node, Edge, NodeId, Result, Error};
use std::collections::HashMap;
use std::path::Path;

const NODES_COL: u8 = 0;
const EDGES_COL: u8 = 1;

/// Database connection using ParityDB
pub struct Database {
    db: Db,
}

impl Database {
    /// Create a new database connection
    pub async fn new(path: &str) -> Result<Self> {
        let path = Path::new(path);
        
        // Configure columns for nodes and edges
        let mut options = Options::with_columns(path, 2);
        
        // Configure node column - hash indexed
        options.columns[NODES_COL as usize] = Column::Hash;
        
        // Configure edge column - hash indexed
        options.columns[EDGES_COL as usize] = Column::Hash;
        
        // Open or create the database
        let db = Db::open_or_create(&options).map_err(|e| Error::Database(format!("{}", e)))?;
        
        Ok(Self { db })
    }

    /// Add a node to the database
    pub async fn add_node(&self, node: &Node) -> Result<()> {
        let value = serde_json::to_vec(&node).map_err(|e| Error::Database(format!("{}", e)))?;
        self.db.commit(vec![(NODES_COL, node.id.as_bytes(), Some(&value))])
            .map_err(|e| Error::Database(format!("{}", e)))?;
        Ok(())
    }

    /// Add an edge to the database
    pub async fn add_edge(&self, edge: &Edge) -> Result<()> {
        // Check for cycles first
        if self.would_create_cycle(&edge.from, &edge.to).await? {
            return Err(Error::CycleDetected {
                from: edge.from.clone(),
                to: edge.to.clone(),
            });
        }

        let key = format!("{}:{}", edge.from, edge.to);
        let value = serde_json::to_vec(&edge).map_err(|e| Error::Database(format!("{}", e)))?;
        
        self.db.commit(vec![(EDGES_COL, key.as_bytes(), Some(&value))])
            .map_err(|e| Error::Database(format!("{}", e)))?;
        
        Ok(())
    }

    /// Get a node by its ID
    pub async fn get_node(&self, id: &NodeId) -> Result<Node> {
        let value = self.db.get(NODES_COL, id.as_bytes())
            .map_err(|e| Error::Database(format!("{}", e)))?
            .ok_or_else(|| Error::NodeNotFound(id.clone()))?;
            
        serde_json::from_slice(&value)
            .map_err(|e| Error::Database(format!("{}", e)))
    }

    /// Get all nodes
    pub async fn get_all_nodes(&self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();
        let iter = self.db.iter(NODES_COL).map_err(|e| Error::Database(format!("{}", e)))?;
        
        for item in iter {
            let (_, value) = item.map_err(|e| Error::Database(format!("{}", e)))?;
            let node: Node = serde_json::from_slice(&value)
                .map_err(|e| Error::Database(format!("{}", e)))?;
            nodes.push(node);
        }
        
        Ok(nodes)
    }

    /// Get all edges
    pub async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        let mut edges = Vec::new();
        let iter = self.db.iter(EDGES_COL).map_err(|e| Error::Database(format!("{}", e)))?;
        
        for item in iter {
            let (_, value) = item.map_err(|e| Error::Database(format!("{}", e)))?;
            let edge: Edge = serde_json::from_slice(&value)
                .map_err(|e| Error::Database(format!("{}", e)))?;
            edges.push(edge);
        }
        
        Ok(edges)
    }

    /// Get all outgoing edges from a node
    pub async fn get_outgoing_edges(&self, node_id: &NodeId) -> Result<Vec<Edge>> {
        let mut edges = Vec::new();
        let iter = self.db.iter(EDGES_COL).map_err(|e| Error::Database(format!("{}", e)))?;
        
        for item in iter {
            let (key, value) = item.map_err(|e| Error::Database(format!("{}", e)))?;
            let key_str = String::from_utf8_lossy(&key);
            
            // Check if this edge starts from our node
            if key_str.starts_with(&format!("{}:", node_id)) {
                let edge: Edge = serde_json::from_slice(&value)
                    .map_err(|e| Error::Database(format!("{}", e)))?;
                edges.push(edge);
            }
        }
        
        Ok(edges)
    }

    /// Check if adding an edge would create a cycle
    async fn would_create_cycle(&self, from: &NodeId, to: &NodeId) -> Result<bool> {
        let mut visited = HashMap::new();
        let mut stack = vec![to.clone()];

        while let Some(current) = stack.pop() {
            if &current == from {
                return Ok(true);
            }

            if !visited.contains_key(&current) {
                visited.insert(current.clone(), true);
                
                // Get outgoing edges
                let edges = self.get_outgoing_edges(&current).await?;
                stack.extend(edges.into_iter().map(|e| e.to));
            }
        }

        Ok(false)
    }
} 