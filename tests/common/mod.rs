use dotenvy::dotenv;
use govuk_prototype_rs;
use once_cell::sync::OnceCell;
use rocket::local::blocking::Client;
use std::env;
use std::sync::Mutex;
use temp_env;

#[cfg(test)]
// Use temp_env to ensure that the database rocket connects to is a dedicated test one rather than
// the development database.

// Since Cargo tests run in parallel we also create a static instance
// and wrap it in a mutex to ensure only one test is spinning up a connection pool at a time
// otherwise each test spins up it's own with ~30 connections and a small/local Postgres instance
// is quickly swamped.
pub fn test_client() -> &'static Mutex<Client> {
    temp_env::with_var("DATABASE_NAME", Some(test_database_name()), || {
        static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
        return INSTANCE.get_or_init(|| {
            Mutex::from(
                Client::tracked(govuk_prototype_rs::rocket()).expect("valid rocket instance"),
            )
        });
    })
}

#[cfg(test)]
fn test_database_name() -> String {
    dotenv().ok();
    let database_name =
        env::var("DATABASE_NAME").expect("No DATABASE_NAME environment variable found");
    format!("{}_test", database_name)
}
