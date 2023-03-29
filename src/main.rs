use crate::auth::{login, login_check, login_page, logout, rocket_uri_macro_login_page};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
};
use rocket_dyn_templates::Template;

mod auth;
mod db;
#[macro_use]
extern crate rocket;

// Proxy traffic to internal website
#[get("/proxy")]
fn proxy(jar: &CookieJar<'_>) -> Redirect {
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

// Launch server with routes and database connection
#[launch]
fn rocket() -> _ {
    let db_file = dotenv::var("DATABASE_FILE").expect("DATABASE_FILE must be set");
    let db = db::Database::new(&db_file).expect("Failed to connect to database");

    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![login_page, login, login_check, proxy, logout])
        .manage(db)
}
