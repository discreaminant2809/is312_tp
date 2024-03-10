use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};
use tokio::sync::RwLock;

mod change_pwd;
mod edit_post;
mod login;
mod posts;
mod search_post;
mod signout;
mod signup;
mod user;
mod view_post;

#[derive(Default, Clone)]
struct Model {
    db: Arc<RwLock<crate::db::Db>>,
}

pub fn router() -> Router {
    let model = Model::default();
    Router::new()
        .nest("/editpost", edit_post::router(model.clone()))
        .route("/searchpost", get(search_post::handler))
        .route("/viewpost", get(view_post::handler))
        .route("/changepwd", put(change_pwd::handler))
        .route("/posts", post(posts::handler))
        .route("/user", post(user::handler))
        .route("/login", post(login::handler))
        .route("/signup", post(signup::handler))
        .route("/signout", post(signout::handler))
        .with_state(model)
}
