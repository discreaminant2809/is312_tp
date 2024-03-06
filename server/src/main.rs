use std::net::SocketAddr;

use axum::{routing::get_service, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod apis;
mod db;
mod session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest("/api", apis::router())
        .layer(CookieManagerLayer::new())
        .fallback(get_service(ServeDir::new("./web")));

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
