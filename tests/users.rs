mod common;

use common::*;
use govuk_prototype_rs::models::user::User;
use rocket::http::{ContentType, Status};
use serde_json;

#[test]
fn show_user() {
    let client = test_client().lock().unwrap();
    let req = client.get("/users/1");
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
    let client = test_client().lock().unwrap();
    let req = client
        .post("/users")
        .header(ContentType::Form)
        .body("name=john%20doe&email=john.doe@example.com&age=28");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::new(303));
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
    let req = client.delete(format!("/users/{}", user.id));
    let response = req.dispatch();
    assert_eq!(response.status(), Status::new(303));
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
