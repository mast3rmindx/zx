use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Represents a block in the DAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Unique identifier for the block
    pub id: String,
    
    /// Hash of the block's data
    pub hash: String,
    
    /// References to parent blocks
    pub parents: Vec<String>,
    
    /// Timestamp when the block was created
    pub timestamp: SystemTime,
    
    /// Block height in the DAG
    pub height: u64,
    
    /// Confidence score (0-100) based on network validation
    pub confidence: u8,
    
    /// Whether this block is part of the k-cluster
    pub in_k_cluster: bool,
    
    /// Network delay measured for this block
    pub network_delay: f64,
}

/// Represents a node in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for the node
    pub id: String,
    
    /// Node's network address
    pub address: String,
    
    /// Last time the node was seen
    pub last_seen: SystemTime,
    
    /// Node's current status
    pub status: NodeStatus,
    
    /// Average network delay to this node
    pub avg_delay: f64,
    
    /// Number of blocks validated by this node
    pub blocks_validated: u64,
}

/// Possible node statuses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Inactive,
    Validating,
    Syncing,
}

impl Block {
    /// Creates a new block with the given parameters
    pub fn new(id: String, hash: String, parents: Vec<String>, height: u64) -> Self {
        Self {
            id,
            hash,
            parents,
            timestamp: SystemTime::now(),
            height,
            confidence: 0,
            in_k_cluster: false,
            network_delay: 0.0,
        }
    }
    
    /// Updates the block's confidence score
    pub fn update_confidence(&mut self, score: u8) {
        self.confidence = score.min(100);
    }
    
    /// Updates the block's network delay measurement
    pub fn update_network_delay(&mut self, delay: f64) {
        self.network_delay = delay;
    }
    
    /// Marks/unmarks the block as part of the k-cluster
    pub fn set_k_cluster(&mut self, in_cluster: bool) {
        self.in_k_cluster = in_cluster;
    }
}

impl Node {
    /// Creates a new node with the given parameters
    pub fn new(id: String, address: String) -> Self {
        Self {
            id,
            address,
            last_seen: SystemTime::now(),
            status: NodeStatus::Active,
            avg_delay: 0.0,
            blocks_validated: 0,
        }
    }
    
    /// Updates the node's status
    pub fn update_status(&mut self, status: NodeStatus) {
        self.status = status;
    }
    
    /// Updates the node's average network delay
    pub fn update_avg_delay(&mut self, delay: f64) {
        self.avg_delay = delay;
    }
    
    /// Increments the number of blocks validated by this node
    pub fn increment_blocks_validated(&mut self) {
        self.blocks_validated += 1;
    }
    
    /// Updates the last seen timestamp to now
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }
} 