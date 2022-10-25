use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub age: u8,
    pub grade: u8,
    pub active: bool,
}

#[derive(FromForm)]
pub struct Filters {
    pub age: u8,
}
