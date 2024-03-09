use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::{db::Post, session::Session};

use super::Model;

#[debug_handler]
pub(crate) async fn handler(
    State(model): State<Model>,
    session: Session,
) -> Result<Json<Value>, Error> {
    let db = model.db.read().await;

    db.get_summarized_posts_by_id(session.user_id())
        .await
        .map(|posts| {
            let post_values = posts
                .map(|(post_id, post)| match post {
                    Post::Draft { title, content } => json!({
                        "id": post_id,
                        "title": title,
                        "content": content
                    }),
                    Post::Published {
                        title,
                        date_num,
                        content,
                    } => json!({
                        "id": post_id,
                        "title": title,
                        "content": content,
                        "dateNum": *date_num,
                    }),
                })
                .collect();

            Json(Value::Array(post_values))
        })
        .ok_or(Error)
}

pub(crate) struct Error;

impl Error {
    const MSG: &'static str = "Invalid user id";
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNAUTHORIZED, Self::MSG).into_response()
    }
}
