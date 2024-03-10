use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::db::Post;

use super::Model;

#[debug_handler]
pub(crate) async fn handler(
    State(model): State<Model>,
    Query(params): Query<Params>,
) -> Result<Json<Value>, Error> {
    let db = model.db.read().await;
    let (
        author,
        Post::Published {
            title,
            date_num,
            content,
            ..
        },
    ) = db.get_post_by_id(params.post_id).await.ok_or(Error)?
    else {
        return Err(Error);
    };

    Ok(Json(json!({
        "author": author,
        "title": title,
        "dateNum": date_num,
        "content": content,
    })))
}

#[derive(Deserialize)]
// #[serde(rename_all = "lowercase")]
pub(crate) struct Params {
    #[serde(rename = "postid")]
    post_id: usize,
}

pub(crate) struct Error;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NOT_FOUND, "No such post").into_response()
    }
}
