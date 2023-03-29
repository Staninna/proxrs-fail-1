use crate::auth::rocket_uri_macro_login_page;
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
};

// Proxy traffic to internal website
#[get("/proxy")]
pub fn proxy_traffic(jar: &CookieJar<'_>) -> Redirect {
    let session = jar.get("session");
    if session.is_none() {
        jar.add(Cookie::new("msg", "Please login"));
        return Redirect::to(uri!(login_page));
    }

    Redirect::to(format!(
        "http://localhost:8000/?session={}",
        session.unwrap().value()
    ))
}
