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

#[derive(Debug, FromForm)]
pub struct Filters {
    pub name: String,
    pub email: String,
    pub age: i32,
}

impl User {
    pub async fn create(db_conn: &Db, name: &str, email: &str, age: i32) -> User {
        let new_user = NewUser {
            name: String::from(name),
            email: String::from(email),
            age: age,
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

    pub async fn all(db_conn: &Db) -> Result<Vec<User>, diesel::result::Error> {
        db_conn.run(move |conn| users::table.load(&mut *conn)).await
    }

    pub async fn get_by_id(db_conn: &Db, id: i32) -> Result<User, diesel::result::Error> {
        db_conn
            .run(move |conn| users::table.find(id).first::<User>(&mut *conn))
            .await
    }
}
