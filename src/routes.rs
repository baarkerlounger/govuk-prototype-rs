use super::Db;
use crate::data::users::USERS;
use crate::models::user::{create_user, Filters, User};

use rocket::form::Form;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::Route;
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![health, index, user, post, user_create]
}

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/html")]
fn user(uuid: &str) -> Template {
    let user = USERS.get(uuid);
    println!("{:?}", user);
    match user {
        Some(u) => Template::render("user", context! { user: &u }),
        None => Template::render("404", context! {}),
    }
}

#[post("/post", data = "<data>")]
fn post(data: Form<Filters>) -> &'static str {
    println!("{:?}", data.age);
    "Post Request"
}

#[post("/user", data = "<data>")]
async fn user_create(data: Form<Filters>, db_conn: Db) -> Created<Json<User>> {
    let user = create_user(&db_conn, &data.name, &data.email, &data.age).await;
    let url = format!("http://localhost:3000/user/{}", user.id);
    Created::new(url).body(Json(user))
}
