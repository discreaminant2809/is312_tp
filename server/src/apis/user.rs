use axum::{debug_handler, extract::State, response::IntoResponse};

use crate::{apis::Model, session::Session};

#[debug_handler]
pub(super) async fn handler(State(model): State<Model>, session: Session) -> Result<String, Error> {
    let db = model.db.read().await;
    let id = db
        .get_username_from_id(session.user_id())
        .await
        .ok_or(Error)?;

    Ok(id[..].to_string())
}

pub(super) struct Error;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
