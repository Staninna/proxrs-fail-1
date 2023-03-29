use crate::db::Database;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::Redirect,
    State,
};
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

// User login form data structure
#[derive(FromForm)]
pub struct User {
    username: String,
    password: String,
}

// Redirect to proxy or login based on session
#[get("/login")]
pub fn login_check(jar: &CookieJar<'_>) -> Redirect {
    let session = jar.get("session");
    if session.is_some() {
        return Redirect::to("/proxy");
    }

    Redirect::to(uri!(login_page))
}

// Render login page with optional message
#[get("/login-page")]
pub fn login_page(jar: &CookieJar<'_>) -> Template {
    let msg = jar.get("msg");
    match msg {
        Some(m) => {
            jar.remove(Cookie::named("msg"));
            Template::render("login", context! { msg: m.value() })
        }
        None => Template::render("login", context! { msg: "" }),
    }
}

// Handle logout and redirect to login with message
#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove(Cookie::named("session"));
    jar.add(Cookie::new("msg", "Logged out"));
    Redirect::to(uri!(login_page))
}

// Handle login, check credentials, set session cookie
#[post("/login", data = "<user>")]
pub async fn login(user: Form<User>, db: &State<Database>, jar: &CookieJar<'_>) -> Redirect {
    let username = user.username.clone();
    let password = user.password.clone();

    let user = match db.get_user_by_username(&username).await {
        Ok(u) => u,
        Err(_) => {
            jar.remove(Cookie::named("session"));
            jar.add(Cookie::new("msg", "Password or username is incorrect"));
            return Redirect::to(uri!(login_page));
        }
    };

    if !db.check_password(&user, &password).await.unwrap() {
        jar.remove(Cookie::named("session"));
        jar.add(Cookie::new("msg", "Password or username is incorrect"));
        return Redirect::to(uri!(login_page));
    }

    jar.add(Cookie::new("session", Uuid::new_v4().to_string()));

    Redirect::to("/proxy")
}
