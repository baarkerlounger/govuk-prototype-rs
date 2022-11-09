#[macro_use]
extern crate rocket;

mod config;
pub mod models;
mod routes;
mod schema;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::{database, diesel};

#[database("govuk_prototype_rs")]
pub struct Db(diesel::PgConnection);

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = Db::get_one(&rocket).await.expect("database connection");
    db.run(|conn| match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}

#[catch(404)]
fn not_found(_req: &Request) -> Template {
    Template::render("404", context! {})
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();

    rocket::custom(config::from_env())
        .attach(Template::fairing())
        .attach(Db::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            run_db_migrations,
        ))
        .register("/", catchers![not_found])
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/", routes::routes())
}
