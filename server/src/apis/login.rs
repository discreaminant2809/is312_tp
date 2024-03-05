use axum::{debug_handler, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use super::Model;

#[debug_handler]
pub(super) async fn handler(
    Extension(model): Extension<Model>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    let db = model.db.lock().await;
    db.auth(&payload.username, &payload.pwd)
        .await
        .map_or_else(Response::Error, |()| Response::Success)
}

#[derive(Deserialize)]
pub(super) struct Payload {
    username: String,
    pwd: String,
}

enum Response {
    Success,
    Error(crate::db::AuthError),
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Success => (StatusCode::OK, "Login successful".to_owned()),
            Response::Error(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
        }
        .into_response()
    }
}
