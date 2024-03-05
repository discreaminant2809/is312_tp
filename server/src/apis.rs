use std::sync::Arc;

use axum::{routing::post, Router};
use tokio::sync::RwLock;

mod login;
mod signup;

#[derive(Default, Clone)]
struct Model {
    db: Arc<RwLock<crate::db::Db>>,
}

const AUTH_TOKEN_KEY: &str = "auth-token";

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login::handler))
        .route("/signup", post(signup::handler))
        .with_state(Model::default())
}
