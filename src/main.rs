use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use knightdag::{db::Database, api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get database path from environment or use default
    let database_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "data/knightdag".to_string());

    // Create database directory if it doesn't exist
    std::fs::create_dir_all(&database_path)?;

    // Create database connection
    let db = Database::new(&database_path).await?;

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the API
    let app = api::create_router(db)
        .layer(cors);

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Listening on {}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
