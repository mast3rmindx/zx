mod api;
mod dag;

use std::sync::Arc;
use axum::Server;
use tower_http::cors::{CorsLayer, Any};
use dag::DAG;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create DAG instance
    let dag = Arc::new(DAG::new(80)); // 80% confidence threshold

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with CORS
    let app = api::routes::create_router(dag)
        .layer(cors);

    // Start server
    let addr = "0.0.0.0:8000".parse().unwrap();
    tracing::info!("Starting server on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
