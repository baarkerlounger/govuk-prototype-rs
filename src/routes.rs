use super::Db;
use crate::models::user::{Filters, User};

use rocket::form::Form;
use rocket::response::status::Created;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{get, post, Config, Route};
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![
        health,
        index,
        user,
        user_create,
        users_index,
        users_new,
        api_user_create,
        delete_user
    ]
}

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[route(GET, uri = "/users/<id>", rank = 1, format = "text/html")]
async fn user(db_conn: Db, id: i32) -> Template {
    let user = User::get_by_id(&db_conn, id).await;
    match user {
        Ok(u) => Template::render("users/show", context! { user: &u }),
        Err(_) => Template::render("404", context! {}),
    }
}

#[get("/users")]
async fn users_index(db_conn: Db) -> Template {
    let users = User::all(&db_conn).await;
    match users {
        Ok(users) => Template::render("users/index", context! {users: &users}),
        Err(_e) => Template::render("404", context! {}),
    }
}

#[get("/users/new")]
async fn users_new() -> Template {
    Template::render("users/new", context! {})
}

#[post("/users", data = "<data>")]
async fn user_create(data: Form<Filters>, db_conn: Db) -> Redirect {
    User::create(&db_conn, &data.name, &data.email, data.age).await;
    Redirect::to(uri!("/users"))
}

#[post("/api/users", data = "<data>")]
async fn api_user_create(data: Form<Filters>, db_conn: Db) -> Created<Json<User>> {
    let user = User::create(&db_conn, &data.name, &data.email, data.age).await;
    let url = format!(
        "http://{}:{}/user/{}",
        Config::ADDRESS,
        Config::PORT,
        user.id
    );
    Created::new(url).body(Json(user))
}

#[delete("/users/<id>")]
async fn delete_user(db_conn: Db, id: i32) -> Redirect {
    let res = User::delete(&db_conn, id).await;
    match res {
        Ok(_t) => Redirect::to(uri!("/users")),
        Err(_e) => Redirect::to(uri!("/users")),
    }
}
