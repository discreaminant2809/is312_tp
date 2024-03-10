use axum::{
    debug_handler,
    extract::{Query, State},
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
) -> Json<Value> {
    let db = model.db.read().await;
    let res = if !params.author.is_empty() {
        db.search_post_by_author(
            &params.keyword,
            &params.author,
            params.since.map(|since| since as _),
        )
        .await
        .map(|(post_id, post)| published_post_to_value(post_id, post, &params.author))
        .collect()
    } else {
        if params.keyword.is_empty() {
            return Json(Value::Array(vec![]));
        }

        db.search_post(&params.keyword, params.since.map(|since| since as _))
            .await
            .map(|(post_id, author, post)| published_post_to_value(post_id, post, author))
            .collect()
    };

    Json(Value::Array(res))
}

#[derive(Deserialize)]
pub(crate) struct Params {
    keyword: String,
    author: String,
    since: Option<u64>,
}

fn published_post_to_value(post_id: usize, post: &Post, author: &str) -> Value {
    let Post::Published {
        title,
        date_num,
        content,
        ..
    } = post
    else {
        unreachable!();
    };

    json!({
        "postId": post_id,
        "title": title,
        "author": author,
        "dateNum": date_num,
        "content": content,
    })
}
