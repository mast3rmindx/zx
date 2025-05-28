use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{State, Path},
};
use std::sync::Arc;
use crate::{Node, Edge, NodeId, Result, Error, db::Database};

/// API state containing the database connection
pub struct ApiState {
    db: Database,
}

/// Create the API router
pub fn create_router(db: Database) -> Router {
    let state = Arc::new(ApiState { db });

    Router::new()
        .route("/nodes", get(get_nodes).post(add_node))
        .route("/nodes/:id", get(get_node))
        .route("/edges", get(get_edges).post(add_edge))
        .route("/nodes/:id/edges", get(get_node_edges))
        .with_state(state)
}

/// Get all nodes
async fn get_nodes(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<Node>>> {
    let nodes = state.db.get_all_nodes().await?;
    Ok(Json(nodes))
}

/// Get a specific node
async fn get_node(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<NodeId>,
) -> Result<Json<Node>> {
    let node = state.db.get_node(&id).await?;
    Ok(Json(node))
}

/// Add a new node
async fn add_node(
    State(state): State<Arc<ApiState>>,
    Json(node): Json<Node>,
) -> Result<Json<()>> {
    state.db.add_node(&node).await?;
    Ok(Json(()))
}

/// Get all edges
async fn get_edges(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<Edge>>> {
    let edges = state.db.get_all_edges().await?;
    Ok(Json(edges))
}

/// Add a new edge
async fn add_edge(
    State(state): State<Arc<ApiState>>,
    Json(edge): Json<Edge>,
) -> Result<Json<()>> {
    state.db.add_edge(&edge).await?;
    Ok(Json(()))
}

/// Get all outgoing edges from a node
async fn get_node_edges(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<NodeId>,
) -> Result<Json<Vec<Edge>>> {
    let edges = state.db.get_outgoing_edges(&id).await?;
    Ok(Json(edges))
} 