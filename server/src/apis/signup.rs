use axum::{debug_handler, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use super::Model;

#[debug_handler]
pub(super) async fn handler(
    Extension(model): Extension<Model>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    username: String,
    pwd: String,
}

enum Response {}
