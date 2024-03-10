use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{
    db::ChangePwdByIdError,
    session::{self, Session},
};

use super::Model;

#[debug_handler]
pub(crate) async fn handler(
    State(model): State<Model>,
    session: Session,
    cookies: Cookies,
    Json(payload): Json<Payload>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    db.change_pwd_by_id(session.user_id(), &payload.reenter_pwd, payload.new_pwd)
        .await?;
    session::cookie::remove(&cookies);
    Ok(())
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    reenter_pwd: String,
    new_pwd: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to change password: {0}")]
pub(crate) struct Error(
    #[from]
    #[source]
    ChangePwdByIdError,
);

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
