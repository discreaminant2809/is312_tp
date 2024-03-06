use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tokio::sync::RwLock;

use crate::mws::require_auth;

mod login;
mod signup;
mod user;

#[derive(Default, Clone)]
struct Model {
    db: Arc<RwLock<crate::db::Db>>,
}

pub fn router() -> Router {
    Router::new()
        .route("/user", get(user::handler))
        .route_layer(middleware::from_fn(require_auth))
        .route("/login", post(login::handler))
        .route("/signup", post(signup::handler))
        .with_state(Model::default())
}
