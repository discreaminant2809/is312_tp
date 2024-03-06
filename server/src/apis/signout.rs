use axum::debug_handler;
use tower_cookies::Cookies;

use crate::session;

#[debug_handler]
pub(super) async fn handler(cookies: Cookies) {
    session::cookie::remove(&cookies);
}
