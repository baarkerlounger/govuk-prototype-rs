#[macro_use]
extern crate rocket;

mod data;
mod models;
mod routes;

use rocket::fs::{relative, FileServer};
use rocket::Request;
use rocket_dyn_templates::{context, Template};

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
        .mount("/", routes::routes())
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
