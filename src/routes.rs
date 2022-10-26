use crate::data::users::USERS;
use crate::models::user::Filters;

use rocket::form::Form;
use rocket::Route;
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![health, start_page, user, post]
}

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[get("/")]
fn start_page() -> Template {
    Template::render("start_page", context! {})
}

#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/html")]
fn user(uuid: &str) -> Template {
    let user = USERS.get(uuid);
    match user {
        Some(u) => Template::render("users", context! { user: &u }),
        None => Template::render("404", context! {}),
    }
}

#[post("/post", data = "<data>")]
fn post(data: Form<Filters>) -> &'static str {
    println!("{:?}", data.age);
    "Post Request"
}
