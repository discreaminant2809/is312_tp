use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod apis;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_methods(Any) // Specify allowed methods, e.g., GET, POST, etc.
        .allow_origin(Any) // Specify allowed origins
        .allow_headers(Any); // Specify allowed headers

    let app = Router::new().merge(apis::router()).layer(cors);

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
