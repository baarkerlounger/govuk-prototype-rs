use dotenvy::dotenv;
use govuk_prototype_rs;
use once_cell::sync::OnceCell;
use rocket::local::blocking::Client;
use std::env;
use std::sync::Mutex;
use temp_env;

#[cfg(test)]
pub fn test_client() -> &'static Mutex<Client> {
    dotenv().ok();
    let database_name =
        env::var("DATABASE_NAME").expect("No DATABASE_NAME environment variable found");
    let test_database_name = format!("{}-test", database_name);
    temp_env::with_var("DATABASE_NAME", Some(test_database_name), || {
        static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
        return INSTANCE.get_or_init(|| {
            Mutex::from(
                Client::tracked(govuk_prototype_rs::rocket()).expect("valid rocket instance"),
            )
        });
    })
}
