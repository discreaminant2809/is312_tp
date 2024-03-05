use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

mod apis;
mod db;
mod mws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", apis::router())
        .layer(CookieManagerLayer::new())
        .layer(cors);

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
