use crate::schema::users;

use rocket::serde::Serialize;
use rocket_sync_db_pools::diesel::Insertable;

#[derive(Debug, Serialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub age: u8,
    pub grade: u8,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
}

#[derive(FromForm)]
pub struct Filters {
    pub age: u8,
}
