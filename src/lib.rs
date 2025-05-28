use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod api;
pub mod dag;
pub mod db;
pub mod frontend;

/// A unique identifier for a node in the DAG
pub type NodeId = String;

/// A node in the directed acyclic graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    /// Unique identifier for the node
    pub id: NodeId,
    /// Arbitrary data associated with the node
    pub data: serde_json::Value,
    /// Metadata about the node
    pub metadata: NodeMetadata,
}

/// Metadata associated with a node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeMetadata {
    /// When the node was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional description of the node
    pub description: Option<String>,
    /// Custom key-value metadata
    pub attributes: HashMap<String, String>,
}

/// An edge in the directed acyclic graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edge {
    /// Source node ID
    pub from: NodeId,
    /// Target node ID
    pub to: NodeId,
    /// Edge weight/cost
    pub weight: f64,
    /// Edge metadata
    pub metadata: EdgeMetadata,
}

/// Metadata associated with an edge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeMetadata {
    /// When the edge was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional description of the edge
    pub description: Option<String>,
    /// Custom key-value metadata
    pub attributes: HashMap<String, String>,
}

/// Error types for the knightdag system
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String), // Changed from ParityDb(#[from] parity_db::Error)
    
    #[error("Node not found: {0}")]
    NodeNotFound(NodeId),
    
    #[error("Edge not found: {from} -> {to}")]
    EdgeNotFound {
        from: NodeId,
        to: NodeId,
    },
    
    #[error("Adding edge would create a cycle: {from} -> {to}")]
    CycleDetected {
        from: NodeId,
        to: NodeId,
    },
}

pub type Result<T> = std::result::Result<T, Error>; 