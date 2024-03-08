use axum::response::Redirect;
use tower_cookies::Cookies;

use crate::session;

// #[debug_handler]
pub(super) async fn handler(cookies: Cookies) -> Redirect {
    session::cookie::remove(&cookies);
    Redirect::to("/login.html")
}
