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
pub fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Template::fairing())
        .attach(Db::fairing())
        .register("/", catchers![not_found])
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/", routes::routes())
}
