mod common;

use common::{setup_database, test_client};
use govuk_prototype_rs::models::user::User;
use govuk_prototype_rs::schema::users::dsl::*;
use govuk_prototype_rs::Db;
use rocket::http::{ContentType, Status};
use rocket::tokio::runtime;
use rocket_sync_db_pools::diesel::prelude::*;
use serde_json;

#[test]
fn show_user() {
    setup_database();
    let client = test_client().lock().unwrap();
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let user: User = rt.block_on(async {
        let db_conn = Db::get_one(&*client.rocket()).await.unwrap();
        db_conn
            .run(move |conn| {
                diesel::insert_into(users)
                    .values((
                        &name.eq("john doe"),
                        &email.eq("john.doe@example.com"),
                        &age.eq(15),
                    ))
                    .get_result(&mut *conn)
                    .expect("success")
            })
            .await
    });
    let req = client.get(format!("/users/{}", user.id));
    let response = req.dispatch();
    let expected_content = "john doe";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}

#[test]
fn create_user() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = test_client().lock().unwrap();
    let original_count: i64 = rt.block_on(async {
        let db_conn = Db::get_one(&*client.rocket()).await.unwrap();
        db_conn
            .run(move |conn| users.count().get_result(&mut *conn).expect("Error"))
            .await
    });
    let req = client
        .post("/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    let new_count: i64 = rt.block_on(async {
        let db_conn = Db::get_one(&*client.rocket()).await.unwrap();
        db_conn
            .run(move |conn| users.count().get_result(&mut *conn).expect("Error"))
            .await
    });
    assert_eq!(response.status(), Status::new(303));
    assert_eq!(new_count, original_count + 1);
}

#[test]
fn api_create_user() {
    let client = test_client().lock().unwrap();
    let req = client
        .post("/api/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Created);
}

#[test]
fn user_index() {
    setup_database();
    let client = test_client().lock().unwrap();
    client
        .post("/users")
        .header(ContentType::Form)
        .body("name=testname23&email=testname23@example.com&age=28")
        .dispatch();
    let req = client.get("/users");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap().contains("testname23"), true);
}

#[test]
fn new_user() {
    let client = test_client().lock().unwrap();
    let req = client.get("/users/new");
    let response = req.dispatch();
    let expected_content = "Create a new user";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap().contains(expected_content),
        true
    );
}

#[test]
fn delete_user() {
    let client = test_client().lock().unwrap();
    let req = client
        .post("/api/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    let response_string = response.into_string().unwrap();
    let user: User = serde_json::from_str(&response_string).unwrap();
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let original_count: i64 = rt.block_on(async {
        let db_conn = Db::get_one(&*client.rocket()).await.unwrap();
        db_conn
            .run(move |conn| users.count().get_result(&mut *conn).expect("Error"))
            .await
    });
    let req = client.delete(format!("/users/{}", user.id));
    let response = req.dispatch();
    let new_count: i64 = rt.block_on(async {
        let db_conn = Db::get_one(&*client.rocket()).await.unwrap();
        db_conn
            .run(move |conn| users.count().get_result(&mut *conn).expect("Error"))
            .await
    });
    assert_eq!(response.status(), Status::new(303));
    assert_eq!(new_count, original_count - 1);
}

#[test]
fn edit_and_update_user() {
    let client = test_client().lock().unwrap();
    let req = client
        .post("/api/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    let response_string = response.into_string().unwrap();
    let user: User = serde_json::from_str(&response_string).unwrap();
    let req = client.get(format!("/users/{}/edit", user.id));
    let response = req.dispatch();
    assert_eq!(
        response
            .into_string()
            .unwrap()
            .contains("john.doe@example.com"),
        true
    );
    let req = client
        .put(format!("/users/{}", user.id))
        .header(ContentType::Form)
        .body("name=mary%20jane&email=different@example.com&age=35");
    let response = req.dispatch();
    assert_eq!(
        response
            .into_string()
            .unwrap()
            .contains("different@example.com"),
        true
    );
}
