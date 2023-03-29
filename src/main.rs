use crate::{
    auth::{login, login_check, login_page, logout},
    proxy::proxy_traffic,
};
use rocket_dyn_templates::Template;

mod auth;
mod db;
mod proxy;
#[macro_use]
extern crate rocket;

// Launch server with routes and database connection
#[launch]
fn rocket() -> _ {
    let db_file = dotenv::var("DATABASE_FILE").expect("DATABASE_FILE must be set");
    let db = db::Database::new(&db_file).expect("Failed to connect to database");

    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![login_page, login, login_check, logout, proxy_traffic],
        )
        .manage(db)
}
