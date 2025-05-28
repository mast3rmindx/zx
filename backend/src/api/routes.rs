use axum::{
    Router,
    routing::get,
    extract::State,
    Json,
};
use std::sync::Arc;
use crate::dag::DAG;

pub fn create_router(dag: Arc<DAG>) -> Router {
    Router::new()
        .route("/blocks", get(get_blocks))
        .route("/blocks/k-cluster", get(get_k_cluster))
        .route("/blocks/confirmed", get(get_confirmed_blocks))
        .route("/nodes/active", get(get_active_nodes))
        .with_state(dag)
}

async fn get_blocks(
    State(dag): State<Arc<DAG>>,
) -> Json<Vec<crate::dag::types::Block>> {
    let blocks = dag.blocks.read().await
        .values()
        .cloned()
        .collect();
    Json(blocks)
}

async fn get_k_cluster(
    State(dag): State<Arc<DAG>>,
) -> Json<Vec<crate::dag::types::Block>> {
    let blocks = dag.get_k_cluster().await.unwrap_or_default();
    Json(blocks)
}

async fn get_confirmed_blocks(
    State(dag): State<Arc<DAG>>,
) -> Json<Vec<crate::dag::types::Block>> {
    let blocks = dag.get_confirmed_blocks().await.unwrap_or_default();
    Json(blocks)
}

async fn get_active_nodes(
    State(dag): State<Arc<DAG>>,
) -> Json<Vec<crate::dag::types::Node>> {
    let nodes = dag.get_active_nodes().await.unwrap_or_default();
    Json(nodes)
} 