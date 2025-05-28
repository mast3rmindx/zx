use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use rand::seq::SliceRandom;
use crate::dag::types::{Block, Node, NodeStatus};

/// Represents the DAG and implements the KnightDAG consensus protocol
pub struct DAG {
    /// All blocks in the DAG
    blocks: Arc<RwLock<HashMap<String, Block>>>,
    
    /// All nodes in the network
    nodes: Arc<RwLock<HashMap<String, Node>>>,
    
    /// Current k-cluster size
    k_cluster_size: Arc<RwLock<usize>>,
    
    /// Minimum confidence threshold for block acceptance
    min_confidence: u8,
}

impl DAG {
    /// Creates a new DAG instance
    pub fn new(min_confidence: u8) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(HashMap::new())),
            nodes: Arc::new(RwLock::new(HashMap::new())),
            k_cluster_size: Arc::new(RwLock::new(10)), // Start with k=10
            min_confidence: min_confidence,
        }
    }
    
    /// Adds a new block to the DAG
    pub async fn add_block(&self, block: Block) -> Result<()> {
        // Validate parent blocks exist
        let blocks = self.blocks.read().await;
        for parent in &block.parents {
            if !blocks.contains_key(parent) {
                anyhow::bail!("Parent block {} not found", parent);
            }
        }
        drop(blocks);
        
        // Add the block
        let mut blocks = self.blocks.write().await;
        blocks.insert(block.id.clone(), block);
        
        // Update k-clusters
        self.update_k_clusters().await?;
        
        Ok(())
    }
    
    /// Updates k-clusters based on network conditions
    async fn update_k_clusters(&self) -> Result<()> {
        let blocks = self.blocks.read().await;
        let nodes = self.nodes.read().await;
        
        // Calculate average network delay
        let avg_delay: f64 = nodes.values()
            .filter(|n| n.status == NodeStatus::Active)
            .map(|n| n.avg_delay)
            .sum::<f64>() / nodes.len() as f64;
            
        // Adjust k-cluster size based on network delay
        let mut k_size = self.k_cluster_size.write().await;
        if avg_delay > 1.0 { // High delay
            *k_size = (*k_size * 12 / 10).min(50); // Increase by 20%, max 50
        } else { // Low delay
            *k_size = (*k_size * 8 / 10).max(5); // Decrease by 20%, min 5
        }
        
        // Select k blocks with highest confidence and lowest network delay
        let mut block_scores: Vec<(&String, f64)> = blocks.iter()
            .map(|(id, block)| {
                let score = block.confidence as f64 * (1.0 / (1.0 + block.network_delay));
                (id, score)
            })
            .collect();
            
        block_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Mark top k blocks as part of k-cluster
        let k = *k_size;
        drop(k_size);
        
        let mut blocks = self.blocks.write().await;
        for block in blocks.values_mut() {
            block.set_k_cluster(false);
        }
        
        for (id, _) in block_scores.iter().take(k) {
            if let Some(block) = blocks.get_mut(*id) {
                block.set_k_cluster(true);
            }
        }
        
        Ok(())
    }
    
    /// Validates a block by checking its parents and network delay
    pub async fn validate_block(&self, block_id: &str, node_id: &str) -> Result<()> {
        let mut blocks = self.blocks.write().await;
        let mut nodes = self.nodes.write().await;
        
        let block = blocks.get_mut(block_id)
            .ok_or_else(|| anyhow::anyhow!("Block not found"))?;
            
        let node = nodes.get_mut(node_id)
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;
            
        // Update node status
        node.update_status(NodeStatus::Validating);
        node.increment_blocks_validated();
        node.update_last_seen();
        
        // Calculate parent confidence
        let parent_confidence = block.parents.iter()
            .filter_map(|p| blocks.get(p))
            .map(|p| p.confidence)
            .min()
            .unwrap_or(0);
            
        // Update block confidence based on parent confidence and network delay
        let confidence = ((parent_confidence as f64 * 0.8 + 
            (100.0 / (1.0 + block.network_delay)) * 0.2) as u8)
            .min(100);
            
        block.update_confidence(confidence);
        
        // Update node status back to active
        node.update_status(NodeStatus::Active);
        
        Ok(())
    }
    
    /// Gets blocks that are part of the current k-cluster
    pub async fn get_k_cluster(&self) -> Result<Vec<Block>> {
        let blocks = self.blocks.read().await;
        Ok(blocks.values()
            .filter(|b| b.in_k_cluster)
            .cloned()
            .collect())
    }
    
    /// Gets all blocks with confidence above the minimum threshold
    pub async fn get_confirmed_blocks(&self) -> Result<Vec<Block>> {
        let blocks = self.blocks.read().await;
        Ok(blocks.values()
            .filter(|b| b.confidence >= self.min_confidence)
            .cloned()
            .collect())
    }
    
    /// Adds a new node to the network
    pub async fn add_node(&self, node: Node) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id.clone(), node);
        Ok(())
    }
    
    /// Updates a node's network delay measurement
    pub async fn update_node_delay(&self, node_id: &str, delay: f64) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.update_avg_delay(delay);
            node.update_last_seen();
        }
        Ok(())
    }
    
    /// Gets all active nodes in the network
    pub async fn get_active_nodes(&self) -> Result<Vec<Node>> {
        let nodes = self.nodes.read().await;
        Ok(nodes.values()
            .filter(|n| n.status == NodeStatus::Active)
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;
    
    #[test]
    fn test_add_block() {
        let dag = DAG::new(80);
        
        // Create genesis block
        let genesis = Block::new(
            "genesis".into(),
            "genesis_hash".into(),
            vec![],
            0
        );
        
        // Add genesis block
        block_on(dag.add_block(genesis.clone())).unwrap();
        
        // Create child block
        let child = Block::new(
            "child".into(),
            "child_hash".into(),
            vec!["genesis".into()],
            1
        );
        
        // Add child block
        block_on(dag.add_block(child.clone())).unwrap();
        
        // Verify blocks were added
        let blocks = block_on(async {
            dag.blocks.read().await.len()
        });
        assert_eq!(blocks, 2);
    }
    
    #[test]
    fn test_k_cluster() {
        let dag = DAG::new(80);
        
        // Add some test nodes
        let node1 = Node::new("node1".into(), "addr1".into());
        let node2 = Node::new("node2".into(), "addr2".into());
        block_on(dag.add_node(node1)).unwrap();
        block_on(dag.add_node(node2)).unwrap();
        
        // Update node delays
        block_on(dag.update_node_delay("node1", 0.5)).unwrap();
        block_on(dag.update_node_delay("node2", 1.5)).unwrap();
        
        // Add some blocks
        for i in 0..20 {
            let block = Block::new(
                format!("block{}", i),
                format!("hash{}", i),
                vec![],
                i as u64
            );
            block_on(dag.add_block(block)).unwrap();
        }
        
        // Validate blocks
        for i in 0..20 {
            block_on(dag.validate_block(&format!("block{}", i), "node1")).unwrap();
        }
        
        // Get k-cluster
        let k_cluster = block_on(dag.get_k_cluster()).unwrap();
        assert!(k_cluster.len() <= 10); // Initial k=10
    }
} 