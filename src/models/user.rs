use crate::schema::users;

use crate::Db;
use rocket::serde::Serialize;
use rocket_sync_db_pools::diesel::prelude::*;

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(FromForm)]
pub struct Filters {
    pub name: String,
    pub email: String,
    pub age: i32,
}

pub async fn create_user(db_conn: &Db, name: &str, email: &str, age: &i32) -> User {
    let new_user = NewUser {
        name: String::from(name),
        email: String::from(email),
        age: age.clone(),
    };
    db_conn
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(&mut *conn)
                .expect("Error saving new post")
        })
        .await
}

pub async fn all_users(db_conn: &Db) -> Result<Vec<User>, diesel::result::Error> {
    db_conn.run(move |conn| users::table.load(&mut *conn)).await
}
