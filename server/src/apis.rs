use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::RwLock;

mod login;
// mod require_auth_mw;
mod signout;
mod signup;
mod user;

#[derive(Default, Clone)]
struct Model {
    db: Arc<RwLock<crate::db::Db>>,
}

pub fn router() -> Router {
    Router::new()
        .route("/user", get(user::handler))
        // .route_layer(middleware::from_fn(require_auth_mw::layer_fn))
        .route("/login", post(login::handler))
        .route("/signup", post(signup::handler))
        .route("/signout", post(signout::handler))
        .with_state(Model::default())
}
