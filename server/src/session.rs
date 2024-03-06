use std::num::ParseIntError;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    RequestPartsExt,
};
use tower_cookies::Cookies;

pub const SESSION_KEY: &str = "session";

pub struct Session {
    user_id: usize,
}

impl Session {
    // pub fn new(user_id: usize) -> Self {
    //     Self { user_id }
    // }

    pub fn user_id(&self) -> usize {
        self.user_id
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Session {
    type Rejection = FromRequestPartsError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts.extract::<Cookies>().await?;
        let user_id = cookies
            .get(SESSION_KEY)
            .ok_or(FromRequestPartsError::NoSessionCookie)?
            .value()
            .parse()?;

        Ok(Self { user_id })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FromRequestPartsError {
    #[error("missing CookieManagerLayer")]
    NoCookieManagerLayer(StatusCode, &'static str),
    #[error("missing session cookie")]
    NoSessionCookie,
    #[error("invalid cookie")]
    InvalidId(
        #[from]
        #[source]
        ParseIntError,
    ),
}

impl From<(StatusCode, &'static str)> for FromRequestPartsError {
    fn from((code, msg): (StatusCode, &'static str)) -> Self {
        Self::NoCookieManagerLayer(code, msg)
    }
}

impl IntoResponse for FromRequestPartsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NoCookieManagerLayer(code, msg) => (code, msg).into_response(),
            Self::NoSessionCookie | Self::InvalidId(_) => {
                (StatusCode::UNAUTHORIZED, "Invalid session").into_response()
            }
        }
    }
}