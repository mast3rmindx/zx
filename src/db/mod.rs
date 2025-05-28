use parity_db::{Db, Options, ColumnOptions, ColId, Value}; // Key is Vec<u8>
use crate::{Node, Edge, NodeId, Result, Error};
use std::{collections::{HashMap, HashSet}, path::Path};

const NODE_COLUMN_ID: ColId = 0;
const EDGE_COLUMN_ID: ColId = 1;

// Helper to map errors to Error::Database(String)
fn map_db_error<E: std::fmt::Display>(e: E) -> Error {
    Error::Database(e.to_string())
}

/// Database connection
pub struct Database {
    db: Db,
}

impl Database {
    /// Create a new database connection
    pub fn new(database_path_str: &str) -> Result<Self> {
        let database_path = Path::new(database_path_str);

        if let Some(parent_dir) = database_path.parent() {
            // Ensure the parent directory exists.
            // create_dir_all is idempotent: it's fine if the directory already exists.
            std::fs::create_dir_all(parent_dir).map_err(|e| Error::Database(format!("Failed to create database directory '{:?}': {}", parent_dir, e)))?;
        }
        
        // Attempting Options::new() as it's the documented constructor for 0.5.1
        // If this fails, there's a deeper toolchain/env issue.
        let options = Options::new( 
            database_path.to_path_buf(),
            vec![ColumnOptions::default(), ColumnOptions::default()]
        );
        
        let db = Db::open_or_create(&options).map_err(map_db_error)?;
        Ok(Self { db })
    }

    /// Add a node to the database
    pub async fn add_node(&self, node: &Node) -> Result<()> {
        let serialized_node: Value = bincode::serialize(node)
            .map_err(map_db_error)?;
        
        let key = node.id.as_bytes().to_vec();
        self.db.commit(vec![(NODE_COLUMN_ID, key, Some(serialized_node))])
            .map_err(map_db_error)?;
        Ok(())
    }

    /// Add an edge to the database
    pub async fn add_edge(&self, edge: &Edge) -> Result<()> {
        let all_edges = self.get_all_edges().await?;
        if self.would_create_cycle(&all_edges, &edge.from, &edge.to)? {
            return Err(Error::CycleDetected {
                from: edge.from.clone(),
                to: edge.to.clone(),
            });
        }

        let edge_key_str = format!("{}:{}", edge.from, edge.to);
        let serialized_edge: Value = bincode::serialize(edge)
            .map_err(map_db_error)?;
        
        let key = edge_key_str.as_bytes().to_vec();
        self.db.commit(vec![(EDGE_COLUMN_ID, key, Some(serialized_edge))])
            .map_err(map_db_error)?;
        Ok(())
    }

    fn would_create_cycle(&self, current_edges: &Vec<Edge>, from_node_id: &NodeId, to_node_id: &NodeId) -> Result<bool> {
        if from_node_id == to_node_id {
            return Ok(true);
        }
        let mut adj_list: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for e in current_edges {
            adj_list.entry(e.from.clone()).or_default().push(e.to.clone());
        }
        let mut visited = HashSet::new();
        self.dfs(to_node_id, from_node_id, &adj_list, &mut visited)
    }
    
    fn dfs(&self, current_node_id: &NodeId, target_node_id: &NodeId, adj_list: &HashMap<NodeId, Vec<NodeId>>, visited: &mut HashSet<NodeId>) -> Result<bool> {
        visited.insert(current_node_id.clone());
        if let Some(neighbors) = adj_list.get(current_node_id) {
            for neighbor_id in neighbors {
                if neighbor_id == target_node_id { return Ok(true); }
                if !visited.contains(neighbor_id) {
                    if self.dfs(neighbor_id, target_node_id, adj_list, visited)? { return Ok(true); }
                }
            }
        }
        Ok(false)
    }

    pub async fn get_node(&self, id: &NodeId) -> Result<Node> {
        match self.db.get(NODE_COLUMN_ID, id.as_bytes()).map_err(map_db_error)? {
            Some(serialized_node_value) => {
                let node: Node = bincode::deserialize(&serialized_node_value)
                    .map_err(map_db_error)?;
                Ok(node)
            }
            None => Err(Error::NodeNotFound(id.clone())),
        }
    }

    pub async fn get_all_nodes(&self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();
        let mut iter = self.db.iter(NODE_COLUMN_ID).map_err(map_db_error)?;
        while let Ok(Some((_key, value_bytes))) = iter.next() { 
            let node: Node = bincode::deserialize(&value_bytes)
                .map_err(map_db_error)?;
            nodes.push(node);
        }
        Ok(nodes)
    }

    pub async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        let mut edges = Vec::new();
        let mut iter = self.db.iter(EDGE_COLUMN_ID).map_err(map_db_error)?;
        while let Ok(Some((_key, value_bytes))) = iter.next() {
            let edge: Edge = bincode::deserialize(&value_bytes)
                .map_err(map_db_error)?;
            edges.push(edge);
        }
        Ok(edges)
    }

    pub async fn get_outgoing_edges(&self, node_id: &NodeId) -> Result<Vec<Edge>> {
        let mut outgoing_edges = Vec::new();
        let mut iter = self.db.iter(EDGE_COLUMN_ID).map_err(map_db_error)?;
        while let Ok(Some((_key, value_bytes))) = iter.next() {
            let edge: Edge = bincode::deserialize(&value_bytes)
                 .map_err(map_db_error)?;
            if edge.from == *node_id {
                outgoing_edges.push(edge);
            }
        }
        Ok(outgoing_edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Node, NodeMetadata, Edge, EdgeMetadata}; 
    use std::collections::HashMap; 
    use chrono::Utc;
    use tempfile::tempdir;

    fn setup_db() -> (Database, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.paritydb");
        let db = Database::new(path.to_str().unwrap()).unwrap(); 
        (db, dir)
    }

    fn create_node(id_str: &str) -> Node { 
        Node {
            id: id_str.to_string(),
            data: serde_json::json!({"test": "data"}),
            metadata: NodeMetadata {
                created_at: Utc::now(),
                description: None,
                attributes: HashMap::new(),
            },
        }
    }

    fn create_edge(from_str: &str, to_str: &str) -> Edge { 
        Edge {
            from: from_str.to_string(),
            to: to_str.to_string(),
            weight: 1.0,
            metadata: EdgeMetadata {
                created_at: Utc::now(),
                description: None,
                attributes: HashMap::new(),
            },
        }
    }

    #[tokio::test]
    async fn test_add_get_node() {
        let (db, _dir) = setup_db(); 
        let node_a = create_node("A");
        db.add_node(&node_a).await.unwrap();
        
        let retrieved_node = db.get_node(&"A".to_string()).await.unwrap();
        assert_eq!(retrieved_node.id, node_a.id);
        assert_eq!(retrieved_node.data, node_a.data);

        let all_nodes = db.get_all_nodes().await.unwrap();
        assert_eq!(all_nodes.len(), 1);
        assert_eq!(all_nodes[0].id, node_a.id);
    }

    #[tokio::test]
    async fn test_get_non_existent_node() {
        let (db, _dir) = setup_db();
        let result = db.get_node(&"Z".to_string()).await;
        assert!(matches!(result, Err(Error::NodeNotFound(_))));
    }

    #[tokio::test]
    async fn test_add_get_edge() {
        let (db, _dir) = setup_db();
        let node_a = create_node("A");
        let node_b = create_node("B");
        db.add_node(&node_a).await.unwrap();
        db.add_node(&node_b).await.unwrap();

        let edge_ab = create_edge("A", "B");
        db.add_edge(&edge_ab).await.unwrap();

        let all_edges = db.get_all_edges().await.unwrap();
        assert_eq!(all_edges.len(), 1);
        assert_eq!(all_edges[0].from, "A");
        assert_eq!(all_edges[0].to, "B");

        let outgoing_edges_a = db.get_outgoing_edges(&"A".to_string()).await.unwrap();
        assert_eq!(outgoing_edges_a.len(), 1);
        assert_eq!(outgoing_edges_a[0].to, "B");

        let outgoing_edges_b = db.get_outgoing_edges(&"B".to_string()).await.unwrap();
        assert_eq!(outgoing_edges_b.len(), 0);
    }
    
    #[tokio::test]
    async fn test_cycle_detection_simple() {
        let (db, _dir) = setup_db();
        let node_a = create_node("A");
        db.add_node(&node_a).await.unwrap();
        
        let edge_aa = create_edge("A", "A");
        let result = db.add_edge(&edge_aa).await;
        assert!(matches!(result, Err(Error::CycleDetected { from, to }) if from == "A" && to == "A"));

        let node_b = create_node("B");
        db.add_node(&node_b).await.unwrap();
        
        let edge_ab = create_edge("A", "B");
        db.add_edge(&edge_ab).await.unwrap();
        
        let edge_ba = create_edge("B", "A"); 
        let result_ba = db.add_edge(&edge_ba).await;
        assert!(matches!(result_ba, Err(Error::CycleDetected { from, to }) if from == "B" && to == "A"));
    }

    #[tokio::test]
    async fn test_cycle_detection_complex() {
        let (db, _dir) = setup_db();
        let nodes_data = vec![("A"), ("B"), ("C"), ("D")]; 
        for node_id_str in nodes_data { 
            db.add_node(&create_node(node_id_str)).await.unwrap();
        }

        db.add_edge(&create_edge("A", "B")).await.unwrap();
        db.add_edge(&create_edge("B", "C")).await.unwrap();
        db.add_edge(&create_edge("C", "D")).await.unwrap(); 
        
        let edge_da = create_edge("D", "A");
        let result = db.add_edge(&edge_da).await;
        assert!(matches!(result, Err(Error::CycleDetected { from, to }) if from == "D" && to == "A"));
    }

    #[tokio::test]
    async fn test_add_valid_edges_no_cycle() {
        let (db, _dir) = setup_db();
         let nodes_data = vec![("A"), ("B"), ("C")]; 
        for node_id_str in nodes_data { 
            db.add_node(&create_node(node_id_str)).await.unwrap();
        }

        db.add_edge(&create_edge("A", "B")).await.unwrap();
        db.add_edge(&create_edge("A", "C")).await.unwrap();
        db.add_edge(&create_edge("B", "C")).await.unwrap();
        
        let all_edges = db.get_all_edges().await.unwrap();
        assert_eq!(all_edges.len(), 3);
    }

    #[test] // No tokio runtime needed for this specific test as Database::new is sync
    fn test_database_new_creates_directory() {
        let base_dir = tempdir().unwrap();
        // Define a path that includes a subdirectory that does not exist yet.
        let db_dir_path = base_dir.path().join("some_new_subdir");
        let db_file_path = db_dir_path.join("test_db.paritydb");
        
        // Ensure the subdirectory does not exist before we call Database::new
        assert!(!db_dir_path.exists(), "Test setup error: Subdirectory already exists.");

        // Attempt to create the database. This should trigger directory creation.
        let db_result = Database::new(db_file_path.to_str().unwrap());

        // Check that the database creation was successful
        assert!(db_result.is_ok(), "Database::new failed: {:?}", db_result.err());
        
        // Check that the directory was actually created
        assert!(db_dir_path.exists(), "Database directory was not created.");
        assert!(db_dir_path.is_dir(), "Database path is not a directory.");
        
        // Optional: Check if ParityDB created its actual files (this is a bit more implementation-dependent)
        // For example, ParityDB might create a manifest file or other files within db_file_path if db_file_path itself is treated as a directory by ParityDB.
        // If db_file_path is a single file, then db_file_path.exists() would be the check.
        // Given the current `Options::new(database_path.to_path_buf(), ...)`
        // it's likely `db_file_path` is treated as a directory by ParityDB.
        assert!(db_file_path.exists(), "ParityDB database directory/file was not created at the specified path.");
        assert!(db_file_path.is_dir(), "ParityDB path is not a directory as expected.");
    }
}