use govuk_prototype_rs;
use once_cell::sync::OnceCell;
use rocket::local::blocking::Client;
use std::sync::Mutex;

pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::from(Client::tracked(govuk_prototype_rs::rocket()).expect("valid rocket instance"))
    })
}
