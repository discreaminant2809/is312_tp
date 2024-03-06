use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::session::{self, Session};

pub async fn require_auth(
    session: Result<Session, session::FromRequestPartsError>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    dbg!(&req);
    session?;

    Ok(next.run(req).await)
}

#[derive(Debug, thiserror::Error)]
#[error("Missing auth cookie")]
pub enum Error {
    InvalidSession(
        #[from]
        #[source]
        session::FromRequestPartsError,
    ),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
    }
}
