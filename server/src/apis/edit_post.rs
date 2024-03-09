use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{self, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    db::{ByIdAndPostIdError, Post},
    session::Session,
};

use super::Model;

pub(crate) fn router(_state: Model) -> Router<Model> {
    Router::new()
        .route("/requestedit/:post_id", post(request_edit))
        .route("/newpost", post(new_post))
        .route("/savechange/:post_id", post(save_change))
        .route("/delete/:post_id", routing::delete(delete))
        .route("/publish", post(publish_new))
        .route("/publish/:post_id", post(publish))
}

#[debug_handler]
async fn delete(
    State(model): State<Model>,
    session: Session,
    Path(post_id): Path<usize>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    db.delete_post_by_id_and_post_id(session.user_id(), post_id)
        .await?;
    Ok(())
}

#[debug_handler]
async fn publish(
    State(model): State<Model>,
    session: Session,
    Path(post_id): Path<usize>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    let post = db
        .get_post_by_id_and_post_id_mut(session.user_id(), post_id)
        .await?;

    post.publish(current_time());
    Ok(())
}

#[debug_handler]
async fn publish_new(
    State(model): State<Model>,
    session: Session,
    Json(post): Json<PostPayload>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    db.new_post_by_id(
        session.user_id(),
        Post::Published {
            title: post.title,
            date_num: current_time(),
            content: post.content,
        },
    )
    .await
    .ok_or(Error::InvalidUserId)?;

    Ok(())
}

#[debug_handler]
async fn request_edit(
    State(model): State<Model>,
    session: Session,
    Path(post_id): Path<usize>,
) -> Result<Json<Value>, Error> {
    let db = model.db.read().await;
    let (title, content) = db
        .get_post_by_id_and_post_id(session.user_id(), post_id)
        .await?
        .title_content();

    Ok(Json(json!({
        "title":title,
        "content":content,
    })))
}

#[debug_handler]
async fn new_post(
    State(model): State<Model>,
    session: Session,
    Json(post): Json<PostPayload>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    db.new_post_by_id(
        session.user_id(),
        Post::Draft {
            title: post.title,
            content: post.content,
        },
    )
    .await
    .ok_or(Error::InvalidUserId)?;

    Ok(())
}

#[debug_handler]
async fn save_change(
    State(model): State<Model>,
    session: Session,
    Path(post_id): Path<usize>,
    Json(payload): Json<PostPayload>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    let (title, content) = db
        .get_post_by_id_and_post_id_mut(session.user_id(), post_id)
        .await?
        .title_content_mut();

    *title = payload.title;
    *content = payload.content;

    Ok(())
}

#[derive(Deserialize)]
struct PostPayload {
    title: String,
    content: Value,
}

pub(crate) enum Error {
    InvalidUserId,
    InvalidPostId,
}

impl From<ByIdAndPostIdError> for Error {
    fn from(e: ByIdAndPostIdError) -> Self {
        match e {
            ByIdAndPostIdError::NoSuchUserId => Self::InvalidUserId,
            ByIdAndPostIdError::NoSuchPostId => Self::InvalidPostId,
        }
    }
}

impl Error {
    const INVALID_USER_ID: &'static str = "Invalid user id";
    const INVALID_POST_ID: &'static str = "Invalid post id";
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidUserId => {
                (StatusCode::UNAUTHORIZED, Self::INVALID_USER_ID).into_response()
            }
            Self::InvalidPostId => (StatusCode::BAD_REQUEST, Self::INVALID_POST_ID).into_response(),
        }
    }
}

fn current_time() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("the server is lag behind the epoch time")
        .as_millis()
}

#[cfg(test)]
mod tests {
    // use crate::apis::edit_post::Kind;

    // #[test]
    // fn what_kind_serde_into() {
    //     println!(
    //         "{:?}",
    //         serde_json::ser::to_string_pretty(&Kind::RequestEdit { post_id: 3 })
    //     )
    // }
}
