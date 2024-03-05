use std::sync::Arc;

use axum::{routing::post, Extension, Router};
use tokio::sync::Mutex;

mod login;
mod signup;

#[derive(Default, Clone)]
struct Model {
    db: Arc<Mutex<crate::db::Db>>,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/login", post(login::handler))
        .layer(Extension(Model::default()))
}
