use reqwest::Client;
use rocket::{
    http::{Cookie, CookieJar},
    response::content::{RawCss, RawHtml},
};

// Proxy traffic to internal website
#[get("/proxy")]
pub async fn proxy_html(jar: &CookieJar<'_>) -> RawHtml<String> {
    let session = jar.get("session");
    if session.is_none() {
        jar.add(Cookie::new("msg", "Please login"));

        let html = include_str!("../templates/redirect.html");
        return RawHtml(html.to_string());
    }

    // Get internal website from environment variable
    let internal_website = dotenv::var("INTERNAL_WEBSITE").expect("INTERNAL_WEBSITE must be set");
    let internal_website = internal_website.trim_end_matches('/');

    // Curl internal website and return response
    let client = Client::new();
    let res = client
        .get(format!("{}/", internal_website))
        .send()
        .await
        .unwrap();
    let html = res.text().await.unwrap();

    RawHtml(html)
}

// Proxy traffic to internal website
#[get("/<path..>")]
pub async fn proxy_css(path: std::path::PathBuf, jar: &CookieJar<'_>) -> RawCss<String> {
    let session = jar.get("session");
    if session.is_none() {
        jar.add(Cookie::new("msg", "Please login"));
        return RawCss(include_str!("../templates/redirect.html").to_string());
    }

    // Get internal website from environment variable
    let internal_website = dotenv::var("INTERNAL_WEBSITE").expect("INTERNAL_WEBSITE must be set");
    let internal_website = internal_website.trim_end_matches('/');

    // Curl internal website and return response
    let client = Client::new();
    let res = client
        .get(format!("{}/{}", internal_website, path.to_str().unwrap()))
        .send()
        .await
        .unwrap();
    let css = res.text().await.unwrap();

    RawCss(css)
}
