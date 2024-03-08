use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse};

use crate::{apis::Model, session::Session};

#[debug_handler]
pub(super) async fn handler(State(model): State<Model>, session: Session) -> Result<String, Error> {
    let db = model.db.read().await;
    let id = db
        .get_username_by_id(session.user_id())
        .await
        .ok_or(Error)?;

    Ok(id[..].to_string())
}

pub(super) struct Error;

impl Error {
    const MSG: &'static str = "Invalid user id";
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNAUTHORIZED, Error::MSG).into_response()
    }
}
