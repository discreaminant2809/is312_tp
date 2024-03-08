use std::sync::Arc;

use axum::{routing::post, Router};
use tokio::sync::RwLock;

mod edit_post;
mod login;
mod posts;
mod signout;
mod signup;
mod user;

#[derive(Default, Clone)]
struct Model {
    db: Arc<RwLock<crate::db::Db>>,
}

pub fn router() -> Router {
    let model = Model::default();
    Router::new()
        .nest("/editpost", edit_post::router(model.clone()))
        .route("/posts", post(posts::handler))
        .route("/user", post(user::handler))
        .route("/login", post(login::handler))
        .route("/signup", post(signup::handler))
        .route("/signout", post(signout::handler))
        .with_state(model)
}
