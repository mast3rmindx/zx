use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::{Node, Edge, NodeId, Result, Error};
use std::collections::HashMap;

/// Database connection pool
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Add a node to the database
    pub async fn add_node(&self, node: &Node) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO nodes (id, data, created_at, description, attributes)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            node.id,
            node.data,
            node.metadata.created_at,
            node.metadata.description,
            serde_json::to_value(&node.metadata.attributes)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Add an edge to the database
    pub async fn add_edge(&self, edge: &Edge) -> Result<()> {
        // Check for cycles first
        let would_create_cycle = sqlx::query_scalar!(
            r#"
            SELECT check_dag_cycle($1, $2) as "would_create_cycle!"
            "#,
            edge.from,
            edge.to
        )
        .fetch_one(&self.pool)
        .await?;

        if would_create_cycle {
            return Err(Error::CycleDetected {
                from: edge.from.clone(),
                to: edge.to.clone(),
            });
        }

        sqlx::query!(
            r#"
            INSERT INTO edges (from_node, to_node, weight, created_at, description, attributes)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            edge.from,
            edge.to,
            edge.weight,
            edge.metadata.created_at,
            edge.metadata.description,
            serde_json::to_value(&edge.metadata.attributes)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a node by its ID
    pub async fn get_node(&self, id: &NodeId) -> Result<Node> {
        let record = sqlx::query!(
            r#"
            SELECT id, data, created_at, description, attributes
            FROM nodes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NodeNotFound(id.clone()))?;

        Ok(Node {
            id: record.id,
            data: record.data,
            metadata: crate::NodeMetadata {
                created_at: record.created_at,
                description: record.description,
                attributes: serde_json::from_value(record.attributes)?,
            },
        })
    }

    /// Get all nodes
    pub async fn get_all_nodes(&self) -> Result<Vec<Node>> {
        let records = sqlx::query!(
            r#"
            SELECT id, data, created_at, description, attributes
            FROM nodes
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let nodes = records
            .into_iter()
            .map(|record| Node {
                id: record.id,
                data: record.data,
                metadata: crate::NodeMetadata {
                    created_at: record.created_at,
                    description: record.description,
                    attributes: serde_json::from_value(record.attributes).unwrap_or_default(),
                },
            })
            .collect();

        Ok(nodes)
    }

    /// Get all edges
    pub async fn get_all_edges(&self) -> Result<Vec<Edge>> {
        let records = sqlx::query!(
            r#"
            SELECT from_node, to_node, weight, created_at, description, attributes
            FROM edges
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let edges = records
            .into_iter()
            .map(|record| Edge {
                from: record.from_node,
                to: record.to_node,
                weight: record.weight,
                metadata: crate::EdgeMetadata {
                    created_at: record.created_at,
                    description: record.description,
                    attributes: serde_json::from_value(record.attributes).unwrap_or_default(),
                },
            })
            .collect();

        Ok(edges)
    }

    /// Get all outgoing edges from a node
    pub async fn get_outgoing_edges(&self, node_id: &NodeId) -> Result<Vec<Edge>> {
        let records = sqlx::query!(
            r#"
            SELECT from_node, to_node, weight, created_at, description, attributes
            FROM edges
            WHERE from_node = $1
            "#,
            node_id
        )
        .fetch_all(&self.pool)
        .await?;

        let edges = records
            .into_iter()
            .map(|record| Edge {
                from: record.from_node,
                to: record.to_node,
                weight: record.weight,
                metadata: crate::EdgeMetadata {
                    created_at: record.created_at,
                    description: record.description,
                    attributes: serde_json::from_value(record.attributes).unwrap_or_default(),
                },
            })
            .collect();

        Ok(edges)
    }
} 