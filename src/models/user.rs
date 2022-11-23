use crate::schema::users;
use crate::Db;
use govuk_notify::NotifyClient;
use reqwest;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::diesel::prelude::*;
use serde_json::{Map, Value};
use std::env;

#[derive(Debug, Deserialize, Serialize, Queryable)]
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
                    .expect("Error saving new user")
            })
            .await
    }

    pub async fn delete(db_conn: &Db, user_id: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        db_conn
            .run(move |conn| diesel::delete(users.filter(id.eq(user_id))).execute(conn))
            .await
    }

    pub async fn update(
        db_conn: &Db,
        uid: i32,
        uname: &str,
        uemail: &str,
        uage: i32,
    ) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let user_id = uid;
        let user_name = String::from(uname);
        let user_email = String::from(uemail);
        let user_age = uage;

        db_conn
            .run(move |conn| {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((name.eq(user_name), email.eq(user_email), age.eq(user_age)))
                    .get_result(&mut *conn)
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

    pub async fn notify(&self) -> Result<reqwest::Response, reqwest::Error> {
        let api_key = env::var("GOVUK_NOTIFY_API_KEY").expect("No Notify API Key found");
        let notify_client = NotifyClient::new(api_key);
        let template_id = String::from("217a419e-6a7d-482a-9596-718b889dffce");
        let mut personalisation = Map::new();
        personalisation.insert(
            "variables".to_string(),
            Value::String("some value".to_string()),
        );

        notify_client
            .send_email(self.email.clone(), template_id, Some(personalisation), None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use rocket::async_test;

    #[async_test]
    async fn notify() {
        dotenv().ok();
        let user = User {
            id: 5,
            name: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            age: 32,
        };
        let response = user.notify().await.unwrap();
        assert_eq!(response.status(), 201);
    }
}
