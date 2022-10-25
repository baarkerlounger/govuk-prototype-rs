#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::serde::Serialize;
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;

#[derive(FromForm)]
struct Filters {
    age: u8,
}

#[derive(Debug, Serialize)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    grade: u8,
    active: bool,
}

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert(
            "3e2dd4ae-3c37-40c6-aa64-7061f284ce28",
            User {
                uuid: String::from("3e2dd4ae-3c37-40c6-aa64-7061f284ce28"),
                name: String::from("John Doe"),
                age: 18,
                grade: 1,
                active: true,
            },
        );
        map
    };
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

#[get("/")]
fn start_page() -> Template {
    Template::render("start_page", context! {})
}

#[catch(404)]
fn not_found(_req: &Request) -> Template {
    Template::render("404", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/", routes![start_page, user, post])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn start_page() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/");
        let response = req.dispatch();
        let expected_content =
            "This kit lets you rapidly create HTML prototypes of GOV.UK services.";
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }

    #[test]
    fn user_page() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/user/3e2dd4ae-3c37-40c6-aa64-7061f284ce28");
        let response = req.dispatch();
        let expected_content = "John Doe";
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }

    #[test]
    fn not_found() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/page_that_doesnt_exist");
        let response = req.dispatch();
        let expected_content = "Page not found";
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }
}
