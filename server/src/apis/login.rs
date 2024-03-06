use std::time::Duration;

use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::{
    cookie::{time::OffsetDateTime, Expiration, SameSite},
    Cookie, Cookies,
};

use crate::session::SESSION_KEY;

use super::Model;

#[debug_handler]
pub(super) async fn handler(
    cookies: Cookies,
    State(model): State<Model>,
    Json(payload): Json<Payload>,
) -> Result<&'static str, Error> {
    let db = model.db.read().await;
    let id = db.auth(&payload.username, &payload.pwd).await?;

    cookies.add(
        Cookie::build((SESSION_KEY, id.to_string()))
            .expires(Expiration::DateTime(
                OffsetDateTime::now_utc() + Duration::from_secs(84600 * 30),
            ))
            .path("/")
            .secure(false)
            .http_only(true)
            .permanent()
            .build(),
    );

    Ok("Login successfully")
}

#[derive(Deserialize)]
pub(super) struct Payload {
    username: String,
    pwd: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Login failed: {0}")]
pub(crate) struct Error(
    #[from]
    #[source]
    crate::db::AuthError,
);

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
    }
}

// enum Response {
//     Success,
//     Error(crate::db::AuthError),
// }

// impl IntoResponse for Response {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             Response::Success => (StatusCode::OK, "Login successful".to_owned()),
//             Response::Error(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
//         }
//         .into_response()
//     }
// }
