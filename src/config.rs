use rocket::config::Config;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

pub fn from_env() -> Figment {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", database_url());
    databases.insert("govuk_prototype_rs", database_config);

    Config::figment().merge(("databases", databases))
}

pub fn database_url() -> String {
    let host = env::var("DATABASE_HOST").expect("No DATABASE_HOST environment variable found");
    let name = env::var("DATABASE_NAME").expect("No DATABASE_NAME environment variable found");
    let port = env::var("DATABASE_PORT").expect("No DATABASE_PORT environment variable found");
    let username =
        env::var("DATABASE_USERNAME").expect("No DATABASE_USERNAME environment variable found");
    let password =
        env::var("DATABASE_PASSWORD").expect("No DATABASE_PASSWORD environment variable found");
    format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, name
    )
}
