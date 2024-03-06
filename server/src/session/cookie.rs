use std::time::Duration;

use tower_cookies::{
    cookie::{time::OffsetDateTime, Expiration},
    Cookie, Cookies,
};

const KEY: &str = "session";

pub fn add(cookies: &Cookies, id: usize) {
    cookies.add(
        Cookie::build((KEY, id.to_string()))
            .expires(Expiration::DateTime(
                OffsetDateTime::now_utc() + Duration::from_secs(84600 * 30),
            ))
            .path("/")
            .secure(false)
            .http_only(true)
            .permanent()
            .build(),
    );
}

pub fn get(cookies: &Cookies) -> Option<Cookie> {
    cookies.get(KEY)
}

pub fn remove(cookies: &Cookies) {
    cookies.remove(Cookie::build(KEY).path("/").build());
}
