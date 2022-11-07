#[macro_use]
extern crate rocket;

mod data;
mod models;
mod routes;
mod schema;

use dotenvy::dotenv;
use rocket::fs::{relative, FileServer};
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::{database, diesel};

#[database("govuk-prototype-rs")]
pub struct Db(diesel::PgConnection);

#[catch(404)]
fn not_found(_req: &Request) -> Template {
    Template::render("404", context! {})
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Template::fairing())
        .attach(Db::fairing())
        .register("/", catchers![not_found])
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/", routes::routes())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn health() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/health");
        let response = req.dispatch();
        let expected_content = "ok";
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap().contains(expected_content),
            true
        );
    }

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client.get("/");
        let response = req.dispatch();
        let expected_content = "Gov UK Design System Prototyping in Rust";
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
    fn create_user() {
        let client = Client::tracked(rocket()).unwrap();
        let req = client
            .post("/user")
            .header(ContentType::Form)
            .body("name=john%20doe&email=john.doe@example.com&age=28");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Created);
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
