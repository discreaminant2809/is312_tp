use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::Model;

#[debug_handler]
pub(super) async fn handler(
    State(model): State<Model>,
    Json(payload): Json<Payload>,
) -> Result<&'static str, Error> {
    let mut db = model.db.write().await;
    let _ = db.register(payload.username, payload.pwd).await?;
    drop(db);

    Ok("Sign up successfully")
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    username: String,
    pwd: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Register failed: {0}")]
pub(crate) struct Error(
    #[from]
    #[source]
    crate::db::RegisterError,
);

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CONFLICT, self.to_string()).into_response()
    }
}
